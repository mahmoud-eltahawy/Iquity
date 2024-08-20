use super::utils::read_markdown;
use axum::Router;
use config::GlobalConfig;
use tower_http::services::ServeDir;

use std::{net::SocketAddr, path::PathBuf, sync::Mutex};

pub struct BackendContext {
    pub port: u16,
    pub slides_path: PathBuf,
    pub slides_home_path: PathBuf,
    pub slides: Mutex<Vec<String>>,
    pub slide_index: Mutex<usize>,
    pub config_path: PathBuf,
    pub config: GlobalConfig,
}

impl BackendContext {
    pub async fn new(path: PathBuf, port: u16) -> Result<Self, String> {
        let (markdown_path, markdown_parent_path) = if path.is_file() {
            (path.clone(), path.parent().unwrap().into())
        } else if path.is_dir() {
            let mut son = path.clone();
            son.push("index.md");
            if son.exists() {
                (son, path)
            } else {
                return Err("can not find index.md".to_string());
            }
        } else {
            return Err("provided path does not exist".to_string());
        };

        let slides = read_markdown(&markdown_path)
            .await
            .map_err(|x| x.to_string())?;

        let config_path = GlobalConfig::config_path().unwrap();
        let config = match GlobalConfig::get(&config_path).await {
            Ok(conf) => conf,
            Err(err) => {
                eprintln!("config init error : {}", err);
                GlobalConfig::default()
            }
        };

        Ok(BackendContext {
            slides_path: markdown_path,
            slides_home_path: markdown_parent_path,
            config_path,
            port,
            slides: Mutex::new(slides),
            slide_index: Mutex::new(0),
            config,
        })
    }

    pub fn serve_assets(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let app = Router::new().nest_service("/", ServeDir::new(&self.slides_home_path));
        tokio::task::spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .expect("failed to bind address");
            axum::serve(listener, app)
                .await
                .expect("failded to serve content");
        });
    }
}
