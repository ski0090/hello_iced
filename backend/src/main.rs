use axum::{Router, routing::get_service};
use std::path::PathBuf;
use tower_http::{services::fs::ServeDir, trace::TraceLayer};

fn get_project_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // backend directory
    manifest_dir.parent().unwrap().to_path_buf() // project root
}

#[tokio::main]
async fn main() {
    let project_root = get_project_root();
    let dist_path = project_root.join("dist");

    // static files service setup
    let static_files_service = ServeDir::new(&dist_path) // NOTE: dist directory
        .append_index_html_on_directories(true) // NOTE: find index.html on directories
        .fallback(ServeDir::new(dist_path.join("index.html"))); // NOTE: if not found, instead of 404, serve index.html

    let app = Router::new()
        .fallback_service(get_service(static_files_service)) // NOTE: serve like root directory
        .layer(TraceLayer::new_for_http()) // NOTE: for logging
        .layer(tower_http::cors::CorsLayer::permissive()); // NOTE: allow all origins

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("서버 시작: http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
