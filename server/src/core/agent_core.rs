use log::{info, error};
use std::thread;
use signal_hook::consts::signal::*;
use std::net::SocketAddr;
use std::sync::Arc;
use axum_server::Handle;
use tokio::sync::Notify;
use lazy_static::lazy_static;

#[cfg(not(target_os = "windows"))]
use signal_hook::iterator::Signals;

use crate::config::database::create_db;
use crate::util::rest::gen_tls_config;
use crate::config::route::init_router;

lazy_static! {
    static ref NOTIFY :Arc<Notify> = Arc::new(Notify::new());
}

#[tokio::main]
pub async fn start_agent() {
    info!("start lifefarm...");

    create_db();

    let router = init_router();


    // let abc: axum::Router = utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
    //     .url("/openapi.json", openapi)
    //     .into();

    info!("Router created.");

    #[cfg(not(target_os = "windows"))]
    {
        // add SIGTERM handler
        let mut signals = Signals::new(&[SIGTERM]).expect("Unable to create signal handler");
        thread::spawn(move || {
            for sig in signals.forever() {
                if sig == SIGTERM {
                    info!("SIGTERM signal received.");
                    shutdown();
                    break;
                }
            }
        });
    }

    let addr = SocketAddr::from(([0,0,0,0], 30443));
    let handle = Handle::new();

    let tls_config = gen_tls_config().await;

    println!("Server is running at https://{}", addr);

    let server = axum_server::bind_rustls(addr, tls_config)
            .handle(handle.clone())
            .serve(router.into_make_service());

    let graceful_shutdown = async move {
        NOTIFY.notified().await;

        info!("graceful shutdown signal received.");

        handle.shutdown();
        Ok::<(), std::io::Error>(())
    };

    if let Err(e) = tokio::try_join!(server, graceful_shutdown) {
        error!("Server error: {:?}", e);
    }

    info!("Server terminated.")
}

pub fn shutdown() {
    NOTIFY.notify_one();
    info!("shutdown notify invoked.");
}
