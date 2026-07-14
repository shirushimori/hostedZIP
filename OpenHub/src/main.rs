mod api;
mod service;

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::response::Html;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;

use wry::WebViewBuilder;

#[tokio::main]
async fn main() {
    // --- Collect CLI args early so we can check dev mode before hiding console ---
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let dev_mode = raw_args.iter().any(|a| a == "--dev");

    // On Windows: hide the console window unless developer mode is requested
    #[cfg(windows)]
    if !dev_mode {
        unsafe extern "system" { fn FreeConsole() -> i32; }
        unsafe { FreeConsole(); }
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    println!("[OpenHub] exe_dir: {}", exe_dir.display());

    let data_path = find_data_json(&exe_dir).unwrap_or_else(|| {
        eprintln!("[OpenHub] ERROR: No applicationdata.json found in data/ directory");
        std::process::exit(1);
    });
    println!("[OpenHub] data_path: {}", data_path.display());

    let web_dir = find_web_dir(&exe_dir);
    let data_dir = find_data_dir(&exe_dir);

    if let Some(ref w) = web_dir {
        println!("[OpenHub] web_dir: {}", w.display());
    } else {
        eprintln!("[OpenHub] WARNING: web/ not found — frontend won't be served");
    }
    if let Some(ref d) = data_dir {
        println!("[OpenHub] data_dir: {}", d.display());
    }

    let state = Arc::new(api::AppState {
        mgr: tokio::sync::Mutex::new(None),
        data_path,
        progress: tokio::sync::Mutex::new(api::InstallProgress::default()),
        running_pid: tokio::sync::Mutex::new(None),
    });

    let api_routes = api::router(state.clone());

    let mut app = Router::new()
        .nest("/api", api_routes)
        .layer(CorsLayer::permissive());

    if let Some(web) = &web_dir {
        app = app.fallback_service(ServeDir::new(web).append_index_html_on_directories(true));
    } else {
        app = app.fallback(|| async {
            Html(r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>OpenHub</title><style>body{background:#141414;color:#eee;font-family:sans-serif;display:flex;align-items:center;justify-content:center;height:100vh;margin:0;flex-direction:column;gap:1rem}code{background:#222;padding:0.25em 0.5em;border-radius:4px;color:#16C55B}</style></head><body><h1>OpenHub</h1><p>Frontend not built. Run:</p><p><code>cd src/web_GUI && pnpm dev</code></p><p>Then pass the Vite URL as an argument:</p><p><code>cargo run -- http://localhost:5173</code></p></body></html>"#)
        });
    }

    if let Some(data_static) = &data_dir {
        app = app.nest("/data", Router::new().fallback_service(ServeDir::new(data_static)));
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Failed to bind API server");

    println!("[OpenHub] API server listening on http://127.0.0.1:3001");

    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("API server failed");
    });

    let server_only = raw_args.iter().any(|a| a == "--server-only");

    let default_url = if web_dir.is_some() {
        "http://localhost:3001".into()
    } else {
        eprintln!("[OpenHub] No frontend found. Start Vite: cd src/web_GUI && pnpm dev");
        eprintln!("[OpenHub] Then run with: cargo run -- http://localhost:5173");
        "http://localhost:3001".into()
    };
    let url = raw_args.into_iter()
        .find(|a| a != "--server-only" && a != "--dev")
        .unwrap_or(default_url);
    println!("[OpenHub] Webview URL: {url}");

    if server_only {
        println!("[OpenHub] Server-only mode — press Ctrl+C to stop");
        loop { tokio::time::sleep(std::time::Duration::from_secs(3600)).await; }
    }

    run_webview(&url);
}

#[cfg(target_os = "linux")]
fn run_webview(url: &str) {
    gtk::init().expect("GTK init failed");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("OpenHub");
    window.set_default_size(1280, 770);
    window.set_resizable(false); // Make Linux window not resizable

    let _webview = match WebViewBuilder::new()
        .with_url(url)
        .build_gtk(&window)
    {
        Ok(wv) => Box::leak(Box::new(wv)),
        Err(e) => {
            eprintln!("Failed to create webview: {e}");
            return;
        }
    };

    window.show_all();
    gtk::main();
}

#[cfg(target_os = "windows")]
fn run_webview(url: &str) {
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
    use winit::window::{Window, WindowId};

    struct WebViewApp {
        url: String,
        _webview: Option<wry::WebView>,
        window: Option<Window>,
    }

    impl ApplicationHandler for WebViewApp {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let window_attributes = Window::default_attributes()
                .with_title("OpenHub")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 770.0))
                .with_resizable(false); // Make Windows window not resizable
            let window = event_loop
                .create_window(window_attributes)
                .expect("Failed to create window");

            let webview = WebViewBuilder::new()
                .with_url(&self.url)
                .build(&window)
                .expect("Failed to create webview");

            self._webview = Some(webview);
            self.window = Some(window);
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: WindowId,
            event: WindowEvent,
        ) {
            if let WindowEvent::CloseRequested = event {
                event_loop.exit();
            }
        }

        fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
            event_loop.set_control_flow(ControlFlow::Poll);
        }
    }

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = WebViewApp {
        url: url.to_string(),
        _webview: None,
        window: None,
    };
    event_loop.run_app(&mut app).expect("Event loop error");
}

fn find_data_json(exe_dir: &PathBuf) -> Option<PathBuf> {
    let candidates = [
        exe_dir.join("data"),
        exe_dir.join("../data"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data"),
    ];
    for dir in &candidates {
        if let Some(path) = find_applicationdata_json(dir) {
            return Some(path);
        }
    }
    None
}

fn find_web_dir(exe_dir: &PathBuf) -> Option<PathBuf> {
    let candidates = [
        exe_dir.join("web"),
        exe_dir.join("../web"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/web_GUI/dist"),
    ];
    for dir in &candidates {
        if dir.join("index.html").exists() {
            return Some(dir.clone());
        }
    }
    None
}

fn find_data_dir(exe_dir: &PathBuf) -> Option<PathBuf> {
    let candidates = [
        exe_dir.join("data"),
        exe_dir.join("../data"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data"),
    ];
    for dir in &candidates {
        if dir.exists() {
            return Some(dir.clone());
        }
    }
    None
}

fn find_applicationdata_json(dir: &PathBuf) -> Option<PathBuf> {
    let target = dir.join("applicationdata.json");
    if target.exists() { Some(target) } else { None }
}
