use axum::{
    Router,
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
};
use axum_prometheus::PrometheusMetricLayer;
use iconator::{get_icon_for_file, get_icon_for_folder};
use std::net::SocketAddr;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/icons/{filename}", get(get_icon))
        .route("/file/{path}", get(get_file))
        .route("/folder/{path}", get(get_folder))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);

    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000)))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn get_file(Path(path): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let id = get_icon_for_file(path);
    match id {
        Some(id) => Ok(format!("http://localhost:3000/icons/{id}.svg")),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_folder(Path(path): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let id = get_icon_for_folder(path);
    match id {
        Some(id) => Ok(format!("http://localhost:3000/icons/{id}.svg")),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_icon(Path(filename): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    ServeFile::new(std::path::Path::new("icons").join(filename))
        .try_call(Request::new(()))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
