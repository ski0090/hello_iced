use axum::{Router, routing::get_service};
use tower_http::{services::fs::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    // 정적 파일 서비스 설정
    let static_files_service = ServeDir::new("../dist")
        .append_index_html_on_directories(true)
        .fallback(ServeDir::new("../dist/index.html"));

    let app = Router::new()
        .fallback_service(get_service(static_files_service))
        .layer(TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("서버 시작: http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
