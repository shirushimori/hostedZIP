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
        .route("/run-tool/{name}", post(run_tool))
        .route("/progress", get(get_progress))
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
    }
}

async fn get_service(State(state): State<Arc<AppState>>) -> Json<SrvResponse> {
    let mut g = state.mgr.lock().await;
    match load(&mut g, &state.data_path) {
        Ok(m) => Json(response(m, None)),
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
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
            Json(SrvResponse {
                data: Some(m.data.clone()),
                installed, version_match: vm,
                local_version: m.local_version(),
                remote_version: remote,
                error: None,
            })
        }
        Err(e) => Json(SrvResponse {
            data: None, installed: false, version_match: false,
            local_version: String::new(), remote_version: String::new(), error: Some(e),
        }),
    }
}

async fn install_service(
    State(state): State<Arc<AppState>>,
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
        }),
    }
}
