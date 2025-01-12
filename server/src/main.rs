extern crate dotenv;

pub mod api;
pub mod config;
pub mod schema;

use std::env;
use config::database::create_db;

const DOTENV_CONTENT: &str = include_str!("../.env");

fn main() {
    load_dotenv();

    create_db();
}

fn load_dotenv() {
    for line in DOTENV_CONTENT.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key, value);
        }
    }
}
