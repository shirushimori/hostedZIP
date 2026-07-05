use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State, Json};
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::service::{ServiceData, ServiceManager};

pub struct AppState {
    pub mgr: Mutex<Option<ServiceManager>>,
    pub data_path: PathBuf,
    pub progress: Mutex<InstallProgress>,
    pub running_pid: Mutex<Option<u32>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstallProgress {
    pub step: String,
    pub progress: u32,
    pub error: Option<String>,
    pub done: bool,
}

impl Default for InstallProgress {
    fn default() -> Self {
        Self { step: "Ready".into(), progress: 0, error: None, done: false }
    }
}

#[derive(Serialize)]
pub struct ProgressResponse {
    pub step: String,
    pub progress: u32,
    pub error: Option<String>,
    pub done: bool,
}

#[derive(Serialize)]
pub struct SrvResponse {
    pub data: Option<ServiceData>,
    pub installed: bool,
    pub version_match: bool,
    pub local_version: String,
    pub remote_version: String,
    pub error: Option<String>,
    pub download_size: Option<String>,
}

async fn get_exact_zip_size(url: &str) -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(4))
        .build()
        .ok()?;
    let resp = client.head(url).send().await.ok()?;
    if let Some(len) = resp.headers().get(reqwest::header::CONTENT_LENGTH) {
        if let Ok(len_str) = len.to_str() {
            if let Ok(bytes) = len_str.parse::<u64>() {
                let mb = bytes as f64 / 1024.0 / 1024.0;
                return Some(format!("{:.2} MB", mb));
            }
        }
    }
    None
}

#[derive(Deserialize)]
pub struct InstallRequest {
    pub password: Option<String>,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/service", get(get_service))
        .route("/refresh", post(refresh_service))
        .route("/install", post(install_service))
        .route("/run", post(run_service))
        .route("/run-tool/:name", post(run_tool))
        .route("/run-tool-admin/:name", post(run_tool_admin))
        .route("/run-tool-terminal/:name", post(run_tool_terminal_handler))
        .route("/progress", get(get_progress))
        .route("/stop", post(stop_service))
        .route("/minimize", post(minimize_window))
        .route("/save-hud", post(save_hud_layout))
        .with_state(state)
}

async fn get_progress(State(state): State<Arc<AppState>>) -> Json<ProgressResponse> {
    let p = state.progress.lock().await;
    Json(ProgressResponse {
        step: p.step.clone(),
        progress: p.progress,
        error: p.error.clone(),
        done: p.done,
    })
}

fn load<'a>(mgr: &'a mut Option<ServiceManager>, path: &PathBuf) -> Result<&'a mut ServiceManager, String> {
    if mgr.is_none() {
        *mgr = Some(ServiceManager::load(path).map_err(|e| e.to_string())?);
    }
    Ok(mgr.as_mut().unwrap())
}

fn response(m: &ServiceManager, err: Option<String>) -> SrvResponse {
    let installed = m.is_installed();
    let local = m.local_version();
    SrvResponse {
        data: Some(m.data.clone()),
        installed,
        version_match: false,
        local_version: local,
        remote_version: String::new(),
        error: err,
        download_size: None,
    }
}

async fn get_service(State(state): State<Arc<AppState>>) -> Json<SrvResponse> {
    let mut g = state.mgr.lock().await;
    match load(&mut g, &state.data_path) {
        Ok(m) => {
            let zip_url = m.data.ServiceDownloadZipSourceURL.clone();
            let size = get_exact_zip_size(&zip_url).await;
            let mut res = response(m, None);
            res.download_size = size;
            Json(res)
        }
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
            download_size: None,
        }),
    }
}

async fn refresh_service(State(state): State<Arc<AppState>>) -> Json<SrvResponse> {
    let mut g = state.mgr.lock().await;
    match load(&mut g, &state.data_path) {
        Ok(m) => {
            let remote = m.fetch_version().await.unwrap_or_default();
            let installed = m.is_installed();
            let vm = installed && m.version_matches(&remote);
            let zip_url = m.data.ServiceDownloadZipSourceURL.clone();
            let size = get_exact_zip_size(&zip_url).await;
            Json(SrvResponse {
                data: Some(m.data.clone()),
                installed, version_match: vm,
                local_version: m.local_version(),
                remote_version: remote,
                error: None,
                download_size: size,
            })
        }
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
            download_size: None,
        }),
    }
}

async fn install_service(
    State(state): State<Arc<AppState>>,
    Json(body): Json<InstallRequest>,
) -> Json<SrvResponse> {
    let mut g = state.mgr.lock().await;
    {
        let mut p = state.progress.lock().await;
        p.step = "Starting...".into();
        p.progress = 0;
        p.error = None;
        p.done = false;
    }
    match load(&mut g, &state.data_path) {
        Ok(m) => {
            m.set_password(body.password.unwrap_or_else(|| "1".into()));
            if m.is_installed() && m.root.exists() {
                let _ = std::fs::remove_dir_all(&m.root);
            }
            match m.full_install(&state.progress).await {
                Ok(()) => Json(response(m, None)),
                Err(e) => Json(response(m, Some(e.to_string()))),
            }
        }
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
            download_size: None,
        }),
    }
}

async fn run_service(State(state): State<Arc<AppState>>) -> Json<SrvResponse> {
    let mut g = state.mgr.lock().await;
    match load(&mut g, &state.data_path) {
        Ok(m) => {
            if let Err(e) = m.run() {
                return Json(response(m, Some(e.to_string())));
            }
            Json(response(m, None))
        }
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
            download_size: None,
        }),
    }
}

async fn run_tool(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Json<SrvResponse> {
    let mut g = state.mgr.lock().await;
    match load(&mut g, &state.data_path) {
        Ok(m) => {
            if let Err(e) = m.run_tool(&name) {
                return Json(response(m, Some(e.to_string())));
            }
            Json(response(m, None))
        }
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
            download_size: None,
        }),
    }
}

#[derive(Serialize)]
pub struct StopResponse {
    pub stopped: bool,
    pub error: Option<String>,
}

async fn stop_service(State(state): State<Arc<AppState>>) -> Json<StopResponse> {
    let mut pid_guard = state.running_pid.lock().await;
    match *pid_guard {
        None => Json(StopResponse { stopped: false, error: Some("No process running".into()) }),
        Some(pid) => {
            #[cfg(windows)]
            let result = unsafe {
                use std::ffi::c_void;
                unsafe extern "system" {
                    fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut c_void;
                    fn TerminateProcess(handle: *mut c_void, exit_code: u32) -> i32;
                    fn CloseHandle(handle: *mut c_void) -> i32;
                }
                let handle = OpenProcess(0x0001, 0, pid); // PROCESS_TERMINATE
                if handle.is_null() {
                    Err(format!("OpenProcess failed for PID {pid}"))
                } else {
                    let ok = TerminateProcess(handle, 1) != 0;
                    CloseHandle(handle);
                    if ok { Ok(()) } else { Err(format!("TerminateProcess failed for PID {pid}")) }
                }
            };
            #[cfg(not(windows))]
            let result: Result<(), String> = {
                use std::process::Command;
                Command::new("kill").arg("-9").arg(pid.to_string()).status()
                    .map(|_| ()).map_err(|e| e.to_string())
            };

            match result {
                Ok(()) => { *pid_guard = None; Json(StopResponse { stopped: true, error: None }) }
                Err(e) => Json(StopResponse { stopped: false, error: Some(e) }),
            }
        }
    }
}

async fn run_tool_admin(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Json<SrvResponse> {
    let mut g = state.mgr.lock().await;
    match load(&mut g, &state.data_path) {
        Ok(m) => {
            // run_tool already uses runas on Windows; on Linux it just spawns
            if let Err(e) = m.run_tool(&name) {
                return Json(response(m, Some(e.to_string())));
            }
            Json(response(m, None))
        }
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
            download_size: None,
        }),
    }
}

#[derive(Serialize)]
pub struct MinimizeResponse { pub ok: bool }

async fn minimize_window(_state: State<Arc<AppState>>) -> Json<MinimizeResponse> {
    #[cfg(windows)]
    {
        unsafe extern "system" {
            fn ShowWindow(hwnd: isize, n_cmd_show: i32) -> i32;
            fn GetForegroundWindow() -> isize;
        }
        unsafe {
            // SW_MINIMIZE = 6
            let hwnd = GetForegroundWindow();
            if hwnd != 0 { ShowWindow(hwnd, 6); }
        }
    }
    Json(MinimizeResponse { ok: true })
}

// ── Tool terminal handler ─────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ToolRunResponse {
    pub done: bool,
    pub error: Option<String>,
}

/// Runs the named tool in a visible elevated terminal window.
/// Blocks until the process exits so the frontend can show "Cleaned" reliably.
async fn run_tool_terminal_handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Json<ToolRunResponse> {
    // Resolve the bat path while holding the lock, then release immediately.
    let bat_path = {
        let mut g = state.mgr.lock().await;
        match load(&mut g, &state.data_path) {
            Ok(m) => {
                match m.data.Tools.iter().find(|t| t.displayName == name) {
                    Some(tool) => m.root.join(&tool.path).to_string_lossy().to_string(),
                    None => {
                        return Json(ToolRunResponse {
                            done: false,
                            error: Some(format!("tool '{}' not found", name)),
                        });
                    }
                }
            }
            Err(e) => {
                return Json(ToolRunResponse { done: false, error: Some(e) });
            }
        }
        // MutexGuard drops here
    };

    // Run in a blocking thread so we don't starve the async runtime.
    let result = tokio::task::spawn_blocking(move || -> Result<(), String> {
        crate::service::ServiceManager::run_bat_in_terminal(&bat_path)
            .map_err(|e| e.to_string())
    })
    .await;

    match result {
        Ok(Ok(())) => Json(ToolRunResponse { done: true, error: None }),
        Ok(Err(e)) => Json(ToolRunResponse { done: false, error: Some(e) }),
        Err(e)     => Json(ToolRunResponse { done: false, error: Some(e.to_string()) }),
    }
}

#[derive(Deserialize)]
pub struct SaveHudRequest {
    pub layout: String,
}

#[derive(Serialize)]
pub struct SaveHudResponse {
    pub ok: bool,
    pub path: String,
    pub error: Option<String>,
}

async fn save_hud_layout(
    Json(body): Json<SaveHudRequest>,
) -> Json<SaveHudResponse> {
    let path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("hud_layout.txt")))
        .unwrap_or_else(|| PathBuf::from("hud_layout.txt"));

    match std::fs::write(&path, &body.layout) {
        Ok(()) => Json(SaveHudResponse {
            ok: true,
            path: path.to_string_lossy().to_string(),
            error: None,
        }),
        Err(e) => Json(SaveHudResponse {
            ok: false,
            path: String::new(),
            error: Some(format!("Failed to write file: {e}")),
        }),
    }
}
