
extern crate dotenv;

pub mod config;
pub mod schema;
pub mod util;
pub mod constants;
pub mod comm;
pub mod entity;
pub mod repository;
pub mod core;

use log::{info, warn, error};
use std::env;

use crate::util::farm_logger;
use crate::util::path;

const DOTENV_CONTENT: &str = include_str!("../.env");

fn main() {
    println!("start lifefarm...");

    load_dotenv();

    println!("{} start.", constants::APP_NAME);

    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", args);

    match farm_logger::setup_logger() {
        Ok(_) => {
            info!("Log initialized.");
        },
        Err(err_msg) => {
            error!("failed to set up logger. {}", err_msg);
            return;
        }
    }

    info!("after setup logger.");

    if args.len() < 2 {
        warn!("Usage: {} /install | /remove | /start | /stop | /con", path::app_name());
        return;
    }

    match args[1].as_str() {
        "/install" => install(),
        "/remove" => remove(),
        "/start" => start(),
        "/stop" => stop(),
        "/con" => console(),
        "/service" => service(), // hidden - only called by Windows Service Manager
        _ => warn!("Unknown command: {}.\nUsage: {} /install | /remove | /start | /stop | /con", args[1], &path::app_name()),
    } 
}

fn install() {

}

fn remove() {

}

fn start() {
    core::agent_core::start_agent();
}

fn stop() {

}

fn service() {
}

fn console() {
    core::agent_core::start_agent();
}

fn load_dotenv() {
    for line in DOTENV_CONTENT.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key, value);
        }
    }
}
