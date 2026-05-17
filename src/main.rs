use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ServiceStatus {
    service: &'static str,
    database_configured: bool,
}

async fn root() -> Json<ServiceStatus> {
    Json(ServiceStatus {
        service: "rust-axum",
        database_configured: env::var("DATABASE_URL").is_ok(),
    })
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("bind HTTP listener");

    let app = Router::new()
        .route("/", get(root))
        .route("/healthz", get(root));

    axum::serve(listener, app).await.expect("serve HTTP");
}
