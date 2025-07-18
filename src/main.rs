use std::sync::Arc;

use crate::app::config::Config;
use crate::app::handlers::Handlers;
use crate::app::repositories::Repositories;
use crate::app::services::Services;
use crate::servers::http::server::run_http_server;
use crate::utils::db::init_primary_db;

mod app;
mod domain;
mod feature;
mod servers;
mod utils;
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let config = Config::new().await;
    let http_server = config.server.clone().unwrap_or_else(|| {
        panic!("HTTP server configuration not found");
    });
 
    let pool = init_primary_db(&config).await.expect("Count not init db");
    let repo = Arc::new(Repositories::new(pool));
    let services = Arc::new(Services::new(repo));
    let handlers = Arc::new(Handlers::new(services));
    run_http_server(&http_server.host, http_server.port, handlers).await;

    Ok(())
}
