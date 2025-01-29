
use log::info;

use axum::{
    http::{Method, StatusCode, Uri, Request}, 
    body::Body,
    response::{IntoResponse, Response}};
use axum::routing::{get, post, put};
use axum::Router;

use tower_http::cors::{Any, CorsLayer};
use tower_serve_static::ServeDir;

use include_dir::{include_dir, Dir, File};

use crate::comm::ui::ui_server::*;

static DIST_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/static_dist");

static API_ENP :&str = "/agent";
static UI_API_ENP :&str= "/front";
static DIST_ENP :&str = "/";

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
            .route(ui_api_enp("/bpnt").as_str(), get(handle_get_bpnt))
            // .route(api_enp("/register").as_str(), post(handle_post_register))
            // .route(api_enp("/info").as_str(), get(handle_get_info))
            // .route(api_enp("/:id/status").as_str(), post(handle_post_status))
            // .route(api_enp("/:id/cmd").as_str(), get(handle_get_cmd))
            // .route(api_enp("/:id/cmd-sent").as_str(), post(handle_post_cmd_sent))
            // .route(api_enp("/:id/result").as_str(), post(handle_post_result))
            // .route(api_enp("/:id/audit").as_str(), post(handle_post_audit));

    // /* UI ENDPOINT */
    // router
    //     .route(ui_api_enp("/asset-id").as_str(), get(handle_get_asst_id))
    //     .route(ui_api_enp("/system-list").as_str(), get(handle_get_system_list))
    //     .route(ui_api_enp("/asset").as_str(), get(handle_get_asst))
    //     .route(ui_api_enp("/user").as_str(), put(handle_put_user))
    //     .route(ui_api_enp("/asst-mgmt").as_str(), put(handle_put_asst_mgmt))
    //     .route(ui_api_enp("/agent").as_str(), get(handle_get_agent))
    //     .route(ui_api_enp("/asset").as_str(), post(handle_post_asst))
    //     .route(ui_api_enp("/:id/validate").as_str(), post(handle_post_validate))
    //     .route(ui_api_enp("/:id/unlock").as_str(), post(handle_post_unlock))
    //     .route(ui_api_enp("/log").as_str(), post(handle_post_intgr_log))
    //     .route(ui_api_enp("/setting").as_str(), get(handle_get_setting))
    //     .route(ui_api_enp("/delete").as_str(), get(handle_get_delete))
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

fn api_enp(append: &str) -> String {
    [API_ENP, append].join("")
}

fn ui_api_enp(append: &str) -> String {
    [UI_API_ENP, append].join("")
}