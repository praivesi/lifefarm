
use log::info;

use axum::{
    http::{Method, StatusCode, Uri, Request}, 
    body::Body,
    response::{IntoResponse, Response}};
use axum::routing::{get, post, put, delete};
use axum::Router;

use tower_http::cors::{Any, CorsLayer};
use tower_serve_static::ServeDir;

use include_dir::{include_dir, Dir};

use crate::comm::ui::ui_server::*;

static DIST_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static_dist");

static UI_API_ENP :&str= "/front";
static DIST_ENP :&str = "/static_dist";

pub fn init_router() -> Router {
    let static_service = ServeDir::new(&DIST_DIR);

    let api_routes = create_api_router();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::OPTIONS])
        .allow_headers(Any);

    axum::Router::new()
        .nest_service(DIST_ENP, static_service)
        .fallback(fallback_handler) // deal all 404 Request
        .merge(api_routes)
        .layer(cors)
}

fn create_api_router() -> Router {
    /* API ENDPOINT */
    Router::new()
            .route(ui_api_enp("/user").as_str(), get(handle_get_user))
            .route(ui_api_enp("/blpt").as_str(), get(handle_get_blpt))
            .route(ui_api_enp("/blpt").as_str(), post(handle_post_blpt))
            .route(ui_api_enp("/blpt/{blpt_id}").as_str(), put(handle_put_blpt))
            .route(ui_api_enp("/blpt/{blpt_id}").as_str(), delete(handle_delete_blpt))
            .route(ui_api_enp("/blpt/{blpt_id}/cell").as_str(), get(handle_get_blpt_cell))
            .route(ui_api_enp("/ftpt").as_str(), get(handle_get_ftpt))
}

// fallback_handler: return 'index.html' when 404 Error occurred
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    // info!("fallback_handler called. (uri: {})", uri);

    match DIST_DIR.get_file("index.html") {
        Some(file) => {
            let contents = file.contents();
            
            // let data = String::from_utf8_lossy(contents);
            // info!("index.html: {}", data);

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Body::from(contents))
                .unwrap()
        }
        None => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "Failed to serve index.html for path: {}",
                    uri
                )))
                .unwrap()
        }
    }
}

fn ui_api_enp(append: &str) -> String {
    [UI_API_ENP, append].join("")
}