
use fern::Dispatch;
use log::error;

use std::fs::OpenOptions;
use std::path::Path;
use std::fs;
use chrono::{Datelike, Local, Timelike};

use crate::util::path;
use crate::constants;

pub fn setup_logger() -> Result<bool, String> {
    match verify_log_file_size() {
        Ok(_) => {},
        Err(e) => {
            error!("checking previous log file size failed.");
            return Err(e.to_string());
        }
    };

    let log_dir = path::log_dir_path();

    match fs::create_dir_all(log_dir.to_string()) {
        Ok(_) => { },
        Err(err) => {
            error!("failed to create directory. (path: {})", log_dir);
            return Err(err.to_string());
        }
    }

    match open_log_file(&path::log_file_path()) {
        Ok(_) => Ok(true),
        Err(e) => {
            error!("open log file failed.");
            return Err(e.to_string())
        }
    }
}

fn verify_log_file_size() -> Result<(), fern::InitError> {
    // let org_log_path = path::log_file_path();
    let org_log_path = "./";

    if true != Path::new(&org_log_path).exists() {
        return Ok(())
    }

    let file_size = fs::metadata(&org_log_path)?.len();

    if constants::MAX_FILE_SIZE < file_size {
        let now = Local::now();
        let new_log_file_name = format!(
            "{}-{}-{}_{}-{}-{}_{}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            path::log_filename()
        );

        let bak_log_path = Path::new(&org_log_path).with_file_name(new_log_file_name);
        fs::rename(org_log_path, bak_log_path)?;
    }

    Ok(())
}

fn open_log_file(log_file_path: &str) -> Result<(), fern::InitError> {
    Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
            record.level(),
            record.target(),
            message
        ))
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stdout())
    .chain(OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_file_path)?)
    .apply()?;

    Ok(())
}