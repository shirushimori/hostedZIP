use std::path::{Path, PathBuf};
use std::process::Command;
use crate::service::ServiceData;
use crate::api::InstallProgress;
use tokio::sync::Mutex;

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

    fn seven_zip_path() -> PathBuf {
        let exe = std::env::current_exe().unwrap_or_default();
        let base = exe.parent().unwrap_or(Path::new("."));
        let tools = base.join(".tools");

        #[cfg(target_os = "windows")]
        { tools.join("7z.exe") }
        #[cfg(not(target_os = "windows"))]
        { tools.join("7zz") }
    }

    async fn ensure_7zip() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = Self::seven_zip_path();

        if path.exists() {
            return Ok(path);
        }

        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent)?;

        #[cfg(target_os = "windows")]
        let url = "https://www.7-zip.org/a/7zr.exe";
        #[cfg(not(target_os = "windows"))]
        let url = "https://github.com/ne7ermore/tools/raw/master/linux/7zz";

        println!("Downloading 7zip from {url} ...");
        let resp = reqwest::get(url).await?;
        let bytes = resp.bytes().await?;
        std::fs::write(&path, &bytes)?;

        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
        }

        println!("7zip ready at {}", path.display());
        Ok(path)
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

        let zip_path = self.root.join("_archive.zip");
        std::fs::write(&zip_path, &bytes)?;

        set_step(progress, "Preparing extraction tool...", 45).await;
        let seven = Self::ensure_7zip().await?;

        set_step(progress, "Extracting archive...", 50).await;
        let mut cmd = Command::new(&seven);
        cmd.arg("x").arg(&zip_path).arg(format!("-o{}", self.root.display())).arg("-y");

        if let Some(ref pw) = self.password {
            cmd.arg(format!("-p{pw}"));
        } else {
            cmd.arg("-p");
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            if stderr.contains("Wrong password") || stdout.contains("Wrong password") {
                set_step(progress, "Wrong password", 0).await;
                return Err("WRONG_PASSWORD".into());
            }
            if stderr.contains("Enter password") || stdout.contains("Enter password") {
                set_step(progress, "Password required", 0).await;
                return Err("PASSWORD_REQUIRED".into());
            }
            set_step(progress, "Extraction failed", 0).await;
            return Err(format!("7zip extraction failed: {stdout} {stderr}").into());
        }

        set_step(progress, "Cleaning up...", 85).await;
        let _ = std::fs::remove_file(&zip_path);

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
        Command::new(&exe).spawn()?;
        Ok(())
    }

    pub fn run_tool(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tool = self.data.Tools.iter()
            .find(|t| t.displayName == name)
            .ok_or_else(|| format!("tool '{}' not found", name))?;
        let path = self.root.join(&tool.path);
        Command::new(&path).spawn()?;
        Ok(())
    }
}
