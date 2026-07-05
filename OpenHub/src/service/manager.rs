use std::io::{Read, Write};
use std::path::{Path, PathBuf};
#[cfg(not(windows))]
use std::process::Command;

use crate::service::ServiceData;
use crate::api::InstallProgress;
use tokio::sync::Mutex;

#[cfg(windows)]
mod win {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::mem;

    unsafe extern "system" {
        fn ShellExecuteW(
            hwnd: isize,
            lpOperation: *const u16,
            lpFile: *const u16,
            lpParameters: *const u16,
            lpDirectory: *const u16,
            nShowCmd: i32,
        ) -> isize;
    }

    // For run-in-terminal: ShellExecuteExW lets us get the process handle and wait
    #[repr(C)]
    struct ShellExecInfo {
        cb_size:      u32,
        f_mask:       u32,
        hwnd:         isize,
        lp_verb:      *const u16,
        lp_file:      *const u16,
        lp_parameters:*const u16,
        lp_directory: *const u16,
        n_show:       i32,
        h_inst_app:   isize,
        lp_id_list:   *mut std::ffi::c_void,
        lp_class:     *const u16,
        hkey_class:   isize,
        dw_hot_key:   u32,
        h_icon:       isize,   // union field
        h_process:    isize,
    }

    unsafe extern "system" {
        fn ShellExecuteExW(info: *mut ShellExecInfo) -> i32;
        fn WaitForSingleObject(h: isize, ms: u32) -> u32;
        fn CloseHandle(h: *mut std::ffi::c_void) -> i32;
    }

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(once(0)).collect()
    }

    pub fn runas(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let wide  = to_wide(path);
        let verb  = to_wide("runas");

        let ret = unsafe {
            ShellExecuteW(0, verb.as_ptr(), wide.as_ptr(), std::ptr::null(), std::ptr::null(), 1)
        };

        if ret <= 32 {
            return Err(format!("Failed to run as admin (code: {ret})").into());
        }
        Ok(())
    }

    /// Run a .bat file elevated in a VISIBLE terminal window and WAIT for it to exit.
    /// Uses cmd.exe as the host so the terminal is a proper console window.
    pub fn runas_in_terminal_and_wait(bat_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Pass to cmd.exe so the terminal stays open and output is visible
        let params = format!("/c \"{}\"", bat_path);

        let verb   = to_wide("runas");
        let file   = to_wide("cmd.exe");
        let params_w = to_wide(&params);

        const SEE_MASK_NOCLOSEPROCESS: u32 = 0x00000040;
        const SW_SHOW: i32 = 5;
        const INFINITE: u32 = 0xFFFF_FFFF;

        let mut info = ShellExecInfo {
            cb_size:       mem::size_of::<ShellExecInfo>() as u32,
            f_mask:        SEE_MASK_NOCLOSEPROCESS,
            hwnd:          0,
            lp_verb:       verb.as_ptr(),
            lp_file:       file.as_ptr(),
            lp_parameters: params_w.as_ptr(),
            lp_directory:  std::ptr::null(),
            n_show:        SW_SHOW,
            h_inst_app:    0,
            lp_id_list:    std::ptr::null_mut(),
            lp_class:      std::ptr::null(),
            hkey_class:    0,
            dw_hot_key:    0,
            h_icon:        0,
            h_process:     0,
        };

        let ok = unsafe { ShellExecuteExW(&mut info) };
        if ok == 0 {
            return Err("ShellExecuteExW failed (UAC denied or path not found)".into());
        }

        // Wait for the cmd.exe (and thus the bat) to exit
        if info.h_process != 0 {
            unsafe {
                WaitForSingleObject(info.h_process, INFINITE);
                CloseHandle(info.h_process as *mut std::ffi::c_void);
            }
        }

        Ok(())
    }

    // end of win module
}

pub struct ServiceManager {
    pub data: ServiceData,
    pub root: PathBuf,
    pub password: Option<String>,
}

impl ServiceManager {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let data: ServiceData = serde_json::from_str(&json)?;

        let root = Self::compute_root(&data.ServiceName);
        Ok(Self { data, root, password: Some("1".into()) })
    }

    fn compute_root(service_name: &str) -> PathBuf {
        let exe = std::env::current_exe().unwrap_or_default();
        let base = exe.parent().unwrap_or(Path::new("."));
        base.join("services").join(service_name).join("service")
    }

    pub fn set_password(&mut self, pw: String) {
        self.password = Some(pw);
    }

    pub fn is_installed(&self) -> bool {
        self.version_file().exists()
    }

    pub fn local_version(&self) -> String {
        std::fs::read_to_string(self.version_file()).unwrap_or_default()
    }

    pub fn version_matches(&self, remote: &str) -> bool {
        self.local_version() == remote
    }

    fn version_file(&self) -> PathBuf {
        self.root.join("version.txt")
    }

    pub async fn fetch_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::get(&self.data.ServiceOnlineVersionURL).await?;
        Ok(resp.text().await?.trim().to_string())
    }

    pub async fn full_install(&self, progress: &Mutex<InstallProgress>) -> Result<(), Box<dyn std::error::Error>> {
        async fn set_step(p: &Mutex<InstallProgress>, step: &str, prog: u32) {
            let mut s = p.lock().await;
            s.step = step.to_string();
            s.progress = prog;
        }

        set_step(progress, "Preparing install directory...", 2).await;
        if self.root.exists() {
            std::fs::remove_dir_all(&self.root)?;
        }
        std::fs::create_dir_all(&self.root)?;

        set_step(progress, "Downloading archive...", 10).await;
        let resp = reqwest::get(&self.data.ServiceDownloadZipSourceURL).await?;
        let bytes = resp.bytes().await?.to_vec();
        set_step(progress, "Archive downloaded", 40).await;

        set_step(progress, "Extracting archive...", 50).await;
        let archive_bytes = bytes.clone();
        let password = self.password.clone().unwrap_or_default();
        let root = self.root.clone();

        tokio::task::spawn_blocking(move || -> Result<(), String> {
            let reader = std::io::Cursor::new(archive_bytes);
            let mut archive = zip::ZipArchive::new(reader)
                .map_err(|e| format!("Failed to open archive: {e}"))?;

            let total = archive.len();
            for i in 0..total {
                let mut entry = archive.by_index_decrypt(i, password.as_bytes())
                    .map_err(|e| format!("Failed to decrypt entry {i}: {e}"))?;

                let entry_name = entry.name();
                let name = entry_name.trim_start_matches('/').trim_start_matches('\\');
                let target = root.join(name);

                if entry.is_dir() {
                    std::fs::create_dir_all(&target)
                        .map_err(|e| format!("Failed to create dir: {e}"))?;
                } else {
                    if let Some(parent) = target.parent() {
                        std::fs::create_dir_all(parent)
                            .map_err(|e| format!("Failed to create parent dir: {e}"))?;
                    }
                    let mut out = std::fs::File::create(&target)
                        .map_err(|e| format!("Failed to create file: {e}"))?;
                    let mut data = Vec::new();
                    entry.read_to_end(&mut data)
                        .map_err(|e| format!("Failed to read entry: {e}"))?;
                    out.write_all(&data)
                        .map_err(|e| format!("Failed to write entry: {e}"))?;
                }
            }
            Ok(())
        }).await
        .map_err(|e| format!("Extraction task panicked: {e}"))?
        .map_err(|e| format!("Extraction failed: {e}"))?;

        set_step(progress, "Cleaning up...", 85).await;

        set_step(progress, "Writing version info...", 90).await;
        let version = self.fetch_version().await?;
        std::fs::write(self.version_file(), &version)?;

        set_step(progress, "Finalizing...", 95).await;
        Ok(())
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let exe = self.root.join("Loader.dist/Loader.exe");
        if !exe.exists() {
            return Err("Loader.dist/Loader.exe not found".into());
        }
        #[cfg(windows)]
        win::runas(&exe.to_string_lossy())?;
        #[cfg(not(windows))]
        Command::new(&exe).spawn()?;
        Ok(())
    }

    pub fn run_tool(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tool = self.data.Tools.iter()
            .find(|t| t.displayName == name)
            .ok_or_else(|| format!("tool '{}' not found", name))?;
        let path = self.root.join(&tool.path);
        #[cfg(windows)]
        win::runas(&path.to_string_lossy())?;
        #[cfg(not(windows))]
        Command::new(&path).spawn()?;
        Ok(())
    }

    /// Static entry-point used by the API handler after the mutex is released.
    /// Runs the bat file at `bat_path` in an elevated visible terminal and waits.
    pub fn run_bat_in_terminal(bat_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(windows)]
        win::runas_in_terminal_and_wait(bat_path)?;

        #[cfg(not(windows))]
        {
            Command::new("bash").arg(bat_path).status()?;
        }

        Ok(())
    }
}
