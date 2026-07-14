#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::sync::mpsc;

const BASE_URL: &str =
    "https://raw.githubusercontent.com/shirushimori/hostedZIP/refs/heads/main/data/Dll";
const DLLS: &[(&str, &str)] = &[
    ("opengl32.dll", "opengl32.dll"),
    ("n32.DLL", "n32.DLL"),
    ("p64.dll", "p64.dll"),
];
const INSTALL_DIR: &str = r"C:\Users\mori\dataExfad";
const ICON_URL: &str =
    "https://raw.githubusercontent.com/shirushimori/shirushimori/refs/heads/main/assets/pfp1.jpg";

const HD_PLAYER_PATHS: &[&str] = &[
    r"C:\Program Files\BlueStacks_nxt\HD-Player.exe",
    r"C:\Program Files\BlueStacks_msi5\HD-Player.exe",
    r"C:\Program Files\Bluestacks msi\HD-Player.exe",
];

const BLUESTACKS_PATHS: &[(&str, &str)] = &[
    ("BlueStacks", r"C:\Program Files\Bluestacks"),
    ("BlueStacks msi", r"C:\Program Files\Bluestacks msi"),
    ("BlueStacks X msi", r"C:\Program Files (x86)\BlueStacks X msi"),
    ("BlueStacks X", r"C:\Program Files (x86)\Bluestacks X"),
];

fn main() -> eframe::Result {
    let icon = load_icon();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([360.0, 500.0])
        .with_resizable(false)
        .with_maximized(false)
        .with_title("Chams Installer");

    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(icon_data);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Chams",
        options,
        Box::new(|_cc| Ok(Box::new(ChamsApp::new()))),
    )
}

fn data_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap_or_default()
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_path_buf()
}

fn icon_path() -> std::path::PathBuf {
    data_dir().join("icon.jpg")
}

fn load_icon() -> Option<egui::IconData> {
    let path = icon_path();
    if !path.exists() {
        download_icon(&path).ok()?;
    }
    let bytes = std::fs::read(&path).ok()?;
    let img = image::load_from_memory(&bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some(egui::IconData {
        width: w,
        height: h,
        rgba: rgba.into_raw(),
    })
}

fn download_icon(dest: &std::path::Path) -> Result<(), String> {
    let bytes = ureq::get(ICON_URL)
        .call()
        .map_err(|e| format!("Failed to download icon: {e}"))?
        .body_mut()
        .read_to_vec()
        .map_err(|e| format!("Failed to read icon body: {e}"))?;
    std::fs::write(dest, &bytes)
        .map_err(|e| format!("Failed to write icon: {e}"))
}

fn is_installed() -> bool {
    std::path::Path::new(INSTALL_DIR)
        .join("Chams.exe")
        .exists()
}

enum AppState {
    Configuring,
    Installing,
    Complete,
}

enum Progress {
    Step { current: u32, total: u32, label: String },
    Done(Result<String, String>),
}

struct ChamsApp {
    state: AppState,
    targets: Vec<Target>,
    new_name: String,
    new_path: String,
    status: String,
    progress: f32,
    progress_label: String,
    rx: Option<mpsc::Receiver<Progress>>,
    show_launch_dialog: bool,
    pending_inject: String,
    inject_status: String,
    info_expanded: bool,
}

struct Target {
    name: String,
    path: String,
    enabled: bool,
}

impl ChamsApp {
    fn new() -> Self {
        let initial_state = if is_installed() {
            AppState::Complete
        } else {
            AppState::Configuring
        };

        Self {
            state: initial_state,
            targets: BLUESTACKS_PATHS
                .iter()
                .map(|(n, p)| Target {
                    name: n.to_string(),
                    path: p.to_string(),
                    enabled: true,
                })
                .collect(),
            new_name: String::new(),
            new_path: String::new(),
            status: String::new(),
            progress: 0.0,
            progress_label: String::new(),
            rx: None,
            show_launch_dialog: false,
            pending_inject: String::new(),
            inject_status: String::new(),
            info_expanded: false,
        }
    }
}

impl eframe::App for ChamsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_progress(ctx);
        self.draw_launch_dialog(ctx);

        match &self.state {
            AppState::Configuring => self.draw_configuring(ctx),
            AppState::Installing => self.draw_installing(ctx),
            AppState::Complete => self.draw_complete(ctx),
        }
    }
}

impl ChamsApp {
    fn handle_progress(&mut self, ctx: &egui::Context) {
        let mut done = false;
        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    Progress::Step { current, total, label } => {
                        self.progress = current as f32 / total as f32;
                        self.progress_label = label;
                    }
                    Progress::Done(result) => {
                        self.status = match result {
                            Ok(msg) => msg,
                            Err(e) => format!("Error: {e}"),
                        };
                        self.progress = 1.0;
                        done = true;
                    }
                }
            }
            ctx.request_repaint();
        }
        if done {
            self.rx = None;
            self.state = AppState::Complete;
        }
    }

    fn draw_launch_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_launch_dialog {
            return;
        }
        egui::Window::new("HD-Player.exe not running")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("HD-Player.exe is not running.\nDo you want to launch it?")
                        .size(15.0),
                );
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("Yes").size(15.0))
                                .min_size(egui::vec2(80.0, 30.0)),
                        )
                        .clicked()
                    {
                        self.show_launch_dialog = false;
                        let inject_name = self.pending_inject.clone();
                        match find_hd_player() {
                            Some(exe) => match launch_hd_player(&exe) {
                                Ok(_) => {
                                    let dll_path =
                                        format!("{INSTALL_DIR}\\DLL\\{inject_name}");
                                    match inject_dll_by_name(&inject_name, &dll_path) {
                                        Ok(msg) => self.inject_status = msg,
                                        Err(e) => {
                                            self.inject_status =
                                                format!("Inject failed: {e}")
                                        }
                                    }
                                }
                                Err(e) => {
                                    self.inject_status = format!("Launch failed: {e}")
                                }
                            },
                            None => self.inject_status = "HD-Player.exe not found".into(),
                        }
                    }
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("No").size(15.0))
                                .min_size(egui::vec2(80.0, 30.0)),
                        )
                        .clicked()
                    {
                        self.show_launch_dialog = false;
                    }
                });
            });
    }

    fn draw_configuring(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.heading(egui::RichText::new("Chams Installer").size(26.0));
                ui.separator();
                ui.add_space(4.0);

                ui.label(egui::RichText::new("Install targets:").size(15.0));
                ui.add_space(4.0);
            });

            egui::Frame::group(ui.style()).show(ui, |ui| {
                for t in &mut self.targets {
                    ui.horizontal(|ui| {
                        ui.add_space(6.0);
                        ui.checkbox(&mut t.enabled, "");
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(t.name.clone()).size(15.0).strong());
                            ui.label(
                                egui::RichText::new(t.path.clone())
                                    .size(12.0)
                                    .color(egui::Color32::GRAY),
                            );
                        });
                    });
                }
            });

            ui.add_space(6.0);

            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Add custom emulator:").size(15.0));
                ui.add_space(3.0);

                ui.horizontal(|ui| {
                    let avail = ui.available_width();
                    ui.add_space((avail - 50.0 - 180.0) / 2.0);
                    ui.label(egui::RichText::new("Name:").size(14.0));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_name)
                            .desired_width(180.0)
                            .font(egui::TextStyle::Monospace),
                    );
                });

                ui.horizontal(|ui| {
                    let avail = ui.available_width();
                    ui.add_space((avail - 50.0 - 180.0) / 2.0);
                    ui.label(egui::RichText::new("Path:").size(14.0));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_path)
                            .desired_width(180.0)
                            .font(egui::TextStyle::Monospace),
                    );
                });

                ui.add_space(3.0);

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("+ Add Target").size(15.0))
                            .min_size(egui::vec2(130.0, 28.0)),
                    )
                    .clicked()
                {
                    let name = self.new_name.trim().to_string();
                    let path = self.new_path.trim().to_string();
                    if !name.is_empty() && !path.is_empty() {
                        self.targets
                            .push(Target { name, path, enabled: true });
                        self.new_name.clear();
                        self.new_path.clear();
                    }
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(4.0);

            ui.vertical_centered(|ui| {
                let any = self.targets.iter().any(|t| t.enabled);
                if ui
                    .add_enabled(
                        any,
                        egui::Button::new(egui::RichText::new("Install").size(18.0))
                            .min_size(egui::vec2(180.0, 40.0)),
                    )
                    .clicked()
                {
                    self.state = AppState::Installing;
                    self.progress = 0.0;
                    self.progress_label = "Starting...".into();
                    self.status.clear();

                    let (tx, rx) = mpsc::channel();
                    self.rx = Some(rx);

                    let targets: Vec<(String, String)> = self
                        .targets
                        .iter()
                        .filter(|t| t.enabled)
                        .map(|t| (t.name.clone(), t.path.clone()))
                        .collect();

                    std::thread::spawn(move || {
                        run_install_with_progress(tx, &targets);
                    });
                }
            });
        });
    }

    fn draw_installing(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading(egui::RichText::new("Installing...").size(26.0));
                ui.add_space(20.0);

                let bar = egui::ProgressBar::new(self.progress)
                    .text(&self.progress_label)
                    .animate(true);
                ui.add(bar);
            });
        });
    }

    fn draw_complete(&mut self, ctx: &egui::Context) {
        let installed = is_installed();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(6.0);
                ui.heading(egui::RichText::new("Chams Installer").size(26.0));
                ui.separator();
                ui.add_space(6.0);

                ui.label(
                    egui::RichText::new("Installation Complete")
                        .size(18.0)
                        .color(egui::Color32::from_rgb(80, 200, 80)),
                );
                ui.add_space(6.0);

                egui::CollapsingHeader::new(
                    egui::RichText::new("Installation Details").size(14.0),
                )
                .default_open(self.info_expanded)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&self.status).size(12.0));
                });

                ui.add_space(16.0);
                ui.label(egui::RichText::new("Inject DLL into HD-Player.exe").size(16.0));
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    let avail = ui.available_width();
                    let btn_w = 150.0;
                    let gap = 16.0;
                    ui.add_space((avail - btn_w * 2.0 - gap) / 2.0);

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("INJECT p64").size(18.0).color(egui::Color32::WHITE),
                            )
                            .min_size(egui::vec2(btn_w, 44.0))
                            .fill(egui::Color32::from_rgb(128, 128, 128)),
                        )
                        .clicked()
                    {
                        self.try_inject("p64.dll");
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("INJECT N32").size(18.0).color(egui::Color32::WHITE),
                            )
                            .min_size(egui::vec2(btn_w, 44.0))
                            .fill(egui::Color32::from_rgb(64, 64, 64)),
                        )
                        .clicked()
                    {
                        self.try_inject("n32.DLL");
                    }
                });

                ui.add_space(10.0);

                if !self.inject_status.is_empty() {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(egui::RichText::new(&self.inject_status).size(13.0));
                    });
                    ui.add_space(4.0);
                }

                if installed {
                    ui.add_space(6.0);

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Uninstall All").size(14.0)
                                    .color(egui::Color32::from_rgb(220, 60, 60)),
                            )
                            .min_size(egui::vec2(120.0, 28.0)),
                        )
                        .clicked()
                    {
                        self.run_uninstall();
                    }
                }
            });

            ui.add_space(4.0);

            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::BOTTOM),
                |ui| {
                    ui.add_space(8.0);
                    if installed {
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("Reinstall").size(16.0))
                                .min_size(egui::vec2(160.0, 40.0)),
                        )
                        .clicked()
                    {
                        self.run_uninstall_clean();
                        self.state = AppState::Configuring;
                        self.inject_status.clear();
                        self.status.clear();
                    }
                    } else {
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new("Back").size(16.0))
                                    .min_size(egui::vec2(160.0, 40.0)),
                            )
                            .clicked()
                        {
                            self.state = AppState::Configuring;
                            self.inject_status.clear();
                            self.status.clear();
                        }
                    }
                },
            );
        });
    }

    fn try_inject(&mut self, dll_name: &str) {
        self.inject_status.clear();
        self.pending_inject = dll_name.to_string();

        match find_process_by_name("HD-Player.exe") {
            Some(_pid) => {
                self.inject_status = format!("Found HD-Player.exe, injecting {dll_name}...");
                let dll_path = format!("{INSTALL_DIR}\\DLL\\{dll_name}");
                match inject_dll_by_name(dll_name, &dll_path) {
                    Ok(msg) => self.inject_status = msg,
                    Err(e) => self.inject_status = format!("Inject failed: {e}"),
                }
            }
            None => {
                self.show_launch_dialog = true;
            }
        }
    }

    fn run_uninstall_clean(&self) {
        let install_dir = std::path::Path::new(INSTALL_DIR);
        let _ = std::fs::remove_dir_all(install_dir);

        let desktop = std::env::var("USERPROFILE")
            .map(std::path::PathBuf::from)
            .map(|p| p.join("Desktop").join("Chams.lnk"));
        if let Ok(path) = desktop {
            let _ = std::fs::remove_file(path);
        }

        let start_menu = std::env::var("APPDATA")
            .map(std::path::PathBuf::from)
            .map(|p| p.join(r"Microsoft\Windows\Start Menu\Programs\Chams.lnk"));
        if let Ok(path) = start_menu {
            let _ = std::fs::remove_file(path);
        }

        let bs_paths = &[
            r"C:\Program Files\Bluestacks\opengl32.dll",
            r"C:\Program Files\Bluestacks msi\opengl32.dll",
            r"C:\Program Files (x86)\BlueStacks X msi\opengl32.dll",
            r"C:\Program Files (x86)\Bluestacks X\opengl32.dll",
        ];
        for p in bs_paths {
            let _ = std::fs::remove_file(p);
        }
    }

    fn run_uninstall(&self) {
        let bat_content = format!(
            r#"@echo off
echo ================================
echo  Chams Uninstaller
echo ================================
echo.
echo Closing Chams.exe...
taskkill /F /IM Chams.exe 2>nul
if %errorlevel%==0 (echo [OK] Process closed) else (echo [SKIP] Process not running)
timeout /t 2 /nobreak >nul

echo.
echo Removing installation directory...
if exist "{install_dir}" (
    rmdir /S /Q "{install_dir}"
    echo [OK] Removed {install_dir}
) else (echo [SKIP] Directory not found)

echo.
echo Removing desktop shortcut...
if exist "%USERPROFILE%\Desktop\Chams.lnk" (
    del "%USERPROFILE%\Desktop\Chams.lnk"
    echo [OK] Removed desktop shortcut
) else (echo [SKIP] Not found)

echo.
echo Removing Start Menu shortcut...
if exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Chams.lnk" (
    del "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Chams.lnk"
    echo [OK] Removed Start Menu shortcut
) else (echo [SKIP] Not found)

echo.
echo Removing opengl32.dll from BlueStacks...
for %%d in (
    "C:\Program Files\Bluestacks"
    "C:\Program Files\Bluestacks msi"
    "C:\Program Files (x86)\BlueStacks X msi"
    "C:\Program Files (x86)\Bluestacks X"
) do (
    if exist "%%d\opengl32.dll" (
        del "%%d\opengl32.dll"
        echo [OK] Removed from %%d
    ) else (echo [SKIP] Not in %%d)
)

echo.
echo ================================
echo  Uninstall complete.
echo ================================
pause
"#,
            install_dir = INSTALL_DIR
        );

        let bat_path = data_dir().join("uninstall.bat");
        let _ = std::fs::write(&bat_path, &bat_content);

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let _ = Command::new("cmd")
                .args(["/C", "start", "", bat_path.to_str().unwrap()])
                .spawn();
            std::thread::sleep(std::time::Duration::from_secs(1));
            std::process::exit(0);
        }
    }
}

fn send(tx: &mpsc::Sender<Progress>, current: u32, total: u32, label: &str) {
    let _ = tx.send(Progress::Step {
        current,
        total,
        label: label.to_string(),
    });
}

fn run_install_with_progress(tx: mpsc::Sender<Progress>, targets: &[(String, String)]) {
    let total_steps = 3 + 1 + 1 + 1 + targets.len() as u32;
    let mut step = 0u32;

    let result = (|| -> Result<String, String> {
        let install_dir = std::path::PathBuf::from(INSTALL_DIR);
        let dll_dir = install_dir.join("DLL");
        std::fs::create_dir_all(&dll_dir)
            .map_err(|e| format!("Failed to create {dll_dir:?}: {e}"))?;

        let mut results = Vec::new();

        for (save_name, url_name) in DLLS {
            step += 1;
            send(&tx, step, total_steps, &format!("Downloading {save_name}..."));

            let url = format!("{BASE_URL}/{url_name}");
            let dest = dll_dir.join(save_name);
            let bytes = ureq::get(&url)
                .call()
                .map_err(|e| format!("Failed to download {url_name}: {e}"))?
                .body_mut()
                .read_to_vec()
                .map_err(|e| format!("Failed to read {url_name} body: {e}"))?;
            std::fs::write(&dest, &bytes)
                .map_err(|e| format!("Failed to write {save_name}: {e}"))?;
            results.push(format!("Downloaded {save_name}"));
        }

        step += 1;
        send(&tx, step, total_steps, "Copying exe...");

        let current_exe = std::env::current_exe()
            .map_err(|e| format!("Failed to get current exe: {e}"))?;
        let exe_dest = install_dir.join(
            current_exe
                .file_name()
                .unwrap_or(std::ffi::OsStr::new("Chams.exe")),
        );
        std::fs::copy(&current_exe, &exe_dest)
            .map_err(|e| format!("Failed to copy exe: {e}"))?;
        results.push(format!("Copied exe to {}", exe_dest.display()));

        step += 1;
        send(&tx, step, total_steps, "Creating shortcuts...");

        let desktop = std::env::var("USERPROFILE")
            .map(std::path::PathBuf::from)
            .map(|p| p.join("Desktop"))
            .unwrap_or_else(|_| std::path::PathBuf::from(r"C:\Users\mori\Desktop"));
        create_shortcut(&exe_dest, &desktop.join("Chams.lnk"))?;
        results.push("Created desktop shortcut".into());

        let start_menu = std::env::var("APPDATA")
            .map(std::path::PathBuf::from)
            .map(|p| p.join(r"Microsoft\Windows\Start Menu\Programs"))
            .unwrap_or_else(|_| {
                std::path::PathBuf::from(
                    r"C:\Users\mori\AppData\Roaming\Microsoft\Windows\Start Menu\Programs",
                )
            });
        std::fs::create_dir_all(&start_menu).ok();
        create_shortcut(&exe_dest, &start_menu.join("Chams.lnk"))?;
        results.push("Created Start Menu shortcut".into());

        for (name, path) in targets {
            step += 1;
            send(&tx, step, total_steps, &format!("Deploying to {name}..."));

            match install_opengl32_to(path) {
                Ok(msg) => results.push(msg),
                Err(e) => results.push(format!("{name}: {e}")),
            }
        }

        Ok(results.join("\n"))
    })();

    let _ = tx.send(Progress::Done(result));
}

fn create_shortcut(
    exe_path: &std::path::Path,
    shortcut_path: &std::path::Path,
) -> Result<(), String> {
    let exe_str = exe_path.to_str().ok_or("Invalid exe path")?;
    let lnk_str = shortcut_path.to_str().ok_or("Invalid shortcut path")?;
    let lnk_dir = shortcut_path
        .parent()
        .ok_or("Invalid shortcut dir")?;

    let ps = format!(
        r#"$s = New-Object -ComObject WScript.Shell; $l = $s.CreateShortcut('{lnk_str}'); $l.TargetPath = '{exe_str}'; $l.IconLocation = '{exe_str},0'; $l.Save()"#
    );

    std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps])
        .current_dir(lnk_dir)
        .output()
        .map_err(|e| format!("Failed to run powershell: {e}"))?;

    Ok(())
}

fn install_opengl32_to(dest: &str) -> Result<String, String> {
    let dll_dir = std::path::PathBuf::from(INSTALL_DIR).join("DLL");
    let src = dll_dir.join("opengl32.dll");
    if !src.exists() {
        return Err("opengl32.dll not found in install dir".into());
    }

    let dest = std::path::Path::new(dest);
    std::fs::create_dir_all(dest).map_err(|e| format!("Failed to create dir: {e}"))?;

    let dst = dest.join("opengl32.dll");
    std::fs::copy(&src, &dst).map_err(|e| format!("Failed to copy: {e}"))?;

    Ok(format!("Deployed opengl32.dll to {}", dest.display()))
}

fn find_process_by_name(name: &str) -> Option<u32> {
    #[cfg(target_os = "windows")]
    {
        win_inject::find_process(name)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = name;
        None
    }
}

fn inject_dll_by_name(dll_name: &str, dll_path: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let pid =
            win_inject::find_process("HD-Player.exe").ok_or("HD-Player.exe not found")?;
        win_inject::inject_dll(pid, dll_path)?;
        Ok(format!("Injected {dll_name} into HD-Player.exe (PID {pid})"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (dll_name, dll_path);
        Err("DLL injection only supported on Windows".into())
    }
}

fn find_hd_player() -> Option<String> {
    for path in HD_PLAYER_PATHS {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}

fn launch_hd_player(exe_path: &str) -> Result<u32, String> {
    #[cfg(target_os = "windows")]
    {
        let child = std::process::Command::new(exe_path)
            .spawn()
            .map_err(|e| format!("Failed to launch HD-Player.exe: {e}"))?;
        Ok(child.id())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = exe_path;
        Err("Launching processes only supported on Windows".into())
    }
}

#[cfg(target_os = "windows")]
mod win_inject {
    type Handle = isize;

    const INVALID_HANDLE: Handle = -1;

    const TH32CS_SNAPPROCESS: u32 = 0x0000_0002;
    const PROCESS_CREATE_THREAD: u32 = 0x0002;
    const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
    const PROCESS_VM_OPERATION: u32 = 0x0008;
    const PROCESS_VM_WRITE: u32 = 0x0020;
    const PROCESS_VM_READ: u32 = 0x0010;
    const MEM_COMMIT: u32 = 0x0000_1000;
    const MEM_RESERVE: u32 = 0x0000_2000;
    const MEM_RELEASE: u32 = 0x0000_8000;
    const PAGE_READWRITE: u32 = 0x04;
    const INFINITE: u32 = 0xFFFF_FFFF;

    #[repr(C)]
    #[allow(non_snake_case)]
    struct ProcessEntry32W {
        dw_size: u32,
        cnt_usage: u32,
        th32_process_id: u32,
        th32_default_heap_id: usize,
        th32_module_id: u32,
        cnt_threads: u32,
        th32_parent_process_id: u32,
        pc_pri_class_base: i32,
        dw_flags: u32,
        sz_exe_file: [u16; 260],
    }

    unsafe extern "system" {
        fn CreateToolhelp32Snapshot(dw_flags: u32, th32_process_id: u32) -> Handle;
        fn Process32FirstW(snapshot: Handle, entry: *mut ProcessEntry32W) -> i32;
        fn Process32NextW(snapshot: Handle, entry: *mut ProcessEntry32W) -> i32;
        fn OpenProcess(desired_access: u32, inherit_handle: i32, process_id: u32) -> Handle;
        fn CloseHandle(object: Handle) -> i32;
        fn VirtualAllocEx(
            process: Handle, address: *mut core::ffi::c_void, size: usize,
            alloc_type: u32, protect: u32,
        ) -> *mut core::ffi::c_void;
        fn VirtualFreeEx(
            process: Handle, address: *mut core::ffi::c_void, size: usize, free_type: u32,
        ) -> i32;
        fn WriteProcessMemory(
            process: Handle, base_address: *mut core::ffi::c_void,
            buffer: *const core::ffi::c_void, size: usize, written: *mut usize,
        ) -> i32;
        fn GetModuleHandleW(name: *const u16) -> Handle;
        fn GetProcAddress(module: Handle, name: *const u8) -> *mut core::ffi::c_void;
        fn CreateRemoteThread(
            process: Handle, attrs: *const core::ffi::c_void, stack: usize,
            start: Option<unsafe extern "system" fn(*mut core::ffi::c_void) -> u32>,
            param: *mut core::ffi::c_void, flags: u32, id: *mut u32,
        ) -> Handle;
        fn WaitForSingleObject(handle: Handle, milliseconds: u32) -> u32;
    }

    fn last_error() -> String {
        std::io::Error::last_os_error().to_string()
    }

    pub fn find_process(name: &str) -> Option<u32> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot == INVALID_HANDLE {
                return None;
            }

            let mut entry: ProcessEntry32W = std::mem::zeroed();
            entry.dw_size = std::mem::size_of::<ProcessEntry32W>() as u32;

            if Process32FirstW(snapshot, &mut entry) == 0 {
                CloseHandle(snapshot);
                return None;
            }

            loop {
                let end = entry
                    .sz_exe_file
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(260);
                let proc_name = String::from_utf16_lossy(&entry.sz_exe_file[..end]);
                if proc_name.eq_ignore_ascii_case(name) {
                    let pid = entry.th32_process_id;
                    CloseHandle(snapshot);
                    return Some(pid);
                }
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }

            CloseHandle(snapshot);
            None
        }
    }

    pub fn inject_dll(pid: u32, dll_path: &str) -> Result<(), String> {
        unsafe {
            let access = PROCESS_CREATE_THREAD
                | PROCESS_QUERY_INFORMATION
                | PROCESS_VM_OPERATION
                | PROCESS_VM_WRITE
                | PROCESS_VM_READ;
            let handle = OpenProcess(access, 0, pid);
            if handle == INVALID_HANDLE {
                return Err(format!("OpenProcess failed (PID {pid}): {}", last_error()));
            }

            let wide: Vec<u16> = dll_path.encode_utf16().chain(std::iter::once(0)).collect();
            let size = wide.len() * 2;

            let alloc = VirtualAllocEx(
                handle,
                std::ptr::null_mut(),
                size,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            );
            if alloc.is_null() {
                CloseHandle(handle);
                return Err(format!("VirtualAllocEx failed: {}", last_error()));
            }

            let mut written = 0usize;
            let ok = WriteProcessMemory(
                handle,
                alloc,
                wide.as_ptr() as *const _,
                size,
                &mut written,
            );
            if ok == 0 {
                VirtualFreeEx(handle, alloc, 0, MEM_RELEASE);
                CloseHandle(handle);
                return Err(format!("WriteProcessMemory failed: {}", last_error()));
            }

            let kernel32_name: Vec<u16> = "kernel32.dll\0".encode_utf16().collect();
            let kernel32 = GetModuleHandleW(kernel32_name.as_ptr());
            let load_library_w =
                GetProcAddress(kernel32, b"LoadLibraryW\0".as_ptr());
            if load_library_w.is_null() {
                VirtualFreeEx(handle, alloc, 0, MEM_RELEASE);
                CloseHandle(handle);
                return Err("GetProcAddress(LoadLibraryW) failed".into());
            }

            let thread = CreateRemoteThread(
                handle,
                std::ptr::null(),
                0,
                Some(std::mem::transmute(load_library_w)),
                alloc,
                0,
                std::ptr::null_mut(),
            );
            if thread == INVALID_HANDLE {
                VirtualFreeEx(handle, alloc, 0, MEM_RELEASE);
                CloseHandle(handle);
                return Err(format!("CreateRemoteThread failed: {}", last_error()));
            }

            WaitForSingleObject(thread, INFINITE);

            VirtualFreeEx(handle, alloc, 0, MEM_RELEASE);
            CloseHandle(thread);
            CloseHandle(handle);
            Ok(())
        }
    }
}
