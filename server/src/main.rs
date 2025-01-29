extern crate dotenv;

pub mod api;
pub mod config;
pub mod schema;
pub mod util;
pub mod constants;
pub mod comm;

use log::{info, error};
use std::env;
use config::database::create_db;
use crate::util::farm_logger;

const DOTENV_CONTENT: &str = include_str!("../.env");

fn main() {
    load_dotenv();

    match farm_logger::setup_logger() {
        Ok(_) => {
            info!("Log initialized.");
        },
        Err(err_msg) => {
            error!("failed to set up logger. {}", err_msg);
            return;
        }
    }

    create_db();
}

fn load_dotenv() {
    for line in DOTENV_CONTENT.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key, value);
        }
    }
}
