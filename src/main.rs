use rapier::{info::ClusterInfo,};
use rapier::handler::get_region;
use actix_web::{web, App, HttpServer};

    
#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let cluster_info = ClusterInfo::new().await;
    let cfg = rapier::config::ApiConfig::new();
  
    log::debug!("Starting rapier on port {}", cfg.server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cluster_info.clone()))
            .service(get_region)
    }).bind(("0.0.0.0", cfg.server_port))?
        .run()
        .await
}
