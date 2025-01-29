use crate::constants;

pub fn app_name() -> String { constants::APP_NAME.to_string() }
pub fn log_filename() -> String { constants::LOG_FILENAME.to_string() }

pub fn working_dir() -> String {
    #[cfg(target_os = "windows")]
    {
        constants::WINDOWS_WORKING_DIR.to_string()
    }

    #[cfg(not(target_os = "windows"))]
    {
        constants::LINUX_UNIX_MAC_WORKING_DIR.to_string()
    }
}

pub fn db_dir_path() -> String {
    #[cfg(target_os = "windows")]
    {
        format!("{}{}", constants::WINDOWS_WORKING_DIR, constants::WINDOWS_DATA_DIR)
    }
    #[cfg(not(target_os = "windows"))]

    {
        format!("{}{}", constants::LINUX_UNIX_MAC_WORKING_DIR, constants::LINUX_UNIX_MAC_DATA_DIR)
    }
}

pub fn log_dir_path() -> String {
    #[cfg(target_os = "windows")]
    {
        format!("{}{}", constants::WINDOWS_WORKING_DIR, constants::WINDOWS_DATA_DIR)
    }

    #[cfg(not(target_os = "windows"))]
    {
        format!("{}{}", constants::LINUX_UNIX_MAC_WORKING_DIR, constants::LINUX_UNIX_MAC_DATA_DIR)
    }
}

pub fn app_exec_path() -> String { format!("{}{}", working_dir(), constants::APP_NAME) }
pub fn log_file_path() -> String { format!("{}{}", log_dir_path(), constants::LOG_FILENAME) }
pub fn db_file_path() -> String { format!("{}{}", db_dir_path(), constants::DB_FILENAME) }

pub fn win_service_path() -> String { format!("{}{}.exe", working_dir(), app_name())}

pub fn linux_daemon_path() -> String { format!("/lib/systemd/system/{}.service", app_name()) }

pub fn freebsd_rc_name() -> String { format!("{}_enable", app_name()) }
pub fn freebsd_rc_path() -> String { format!("/usr/local/etc/rc.d/{}", app_name()) }
pub fn freebsd_rc_conf_path() -> String { format!("/etc/rc.conf") }
pub fn freebsd_pid_file_path() -> String {format!("/var/run/{}.pid", app_name()) }