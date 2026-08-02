
use axum::{
    http::{Method, StatusCode},
    body::Body,
    response::{IntoResponse, Response}};
use axum::routing::{get, post, put, delete};
use axum::Router;

use tower::ServiceExt;
use tower_http::cors::{Any, CorsLayer};
use tower_serve_static::ServeDir;

use include_dir::{include_dir, Dir};

use crate::comm::ui::ui_server::*;

static DIST_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static_dist");

static UI_API_ENP :&str= "/front";

pub fn init_router() -> Router {
    let api_routes = create_api_router();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::OPTIONS])
        .allow_headers(Any);

    axum::Router::new()
        .fallback_service(get(static_or_index)) // serve static files, or index.html when not found
        .merge(api_routes)
        .layer(cors)
}

// static_or_index: serve a file from DIST_DIR if it exists, otherwise fall back to index.html
async fn static_or_index(req: axum::extract::Request) -> Response {
    let static_service = ServeDir::new(&DIST_DIR);

    match static_service.oneshot(req).await {
        Ok(res) if res.status() != StatusCode::NOT_FOUND => res.into_response(),
        _ => fallback_handler().await.into_response(),
    }
}

fn create_api_router() -> Router {
    /* API ENDPOINT */
    Router::new()
            .route(ui_api_enp("/user").as_str(), get(handle_get_user))
            .route(ui_api_enp("/user").as_str(), put(handle_put_user))
            .route(ui_api_enp("/blpt").as_str(), get(handle_get_blpt))
            .route(ui_api_enp("/blpt").as_str(), post(handle_post_blpt))
            .route(ui_api_enp("/blpt/{blpt_id}").as_str(), put(handle_put_blpt))
            .route(ui_api_enp("/blpt/{blpt_id}").as_str(), delete(handle_delete_blpt))
            .route(ui_api_enp("/blpt/{blpt_id}/cell").as_str(), get(handle_get_blpt_cell))
            .route(ui_api_enp("/blpt/{blpt_id}/cell").as_str(), post(handle_post_blpt_cell))
            .route(ui_api_enp("/ftpt").as_str(), get(handle_get_ftpt))
            .route(ui_api_enp("/lifefarm").as_str(), get(handle_get_lifefarm))
}

// fallback_handler: return 'index.html' when 404 Error occurred
async fn fallback_handler() -> impl IntoResponse {
    match DIST_DIR.get_file("index.html") {
        Some(file) => {
            let contents = file.contents();

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Body::from(contents))
                .unwrap()
        }
        None => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to serve index.html"))
                .unwrap()
        }
    }
}

fn ui_api_enp(append: &str) -> String {
    [UI_API_ENP, append].join("")
}