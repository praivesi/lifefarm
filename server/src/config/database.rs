extern crate lazy_static;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;
use lazy_static::lazy_static;
// use log::{error, trace};
use std::env;
use std::sync::Mutex;

use include_dir::{include_dir, Dir};

//
// Singleton - SQLITE can only have 1 connection
//
lazy_static! {
    static ref DB_CONNECTION: Mutex<SqliteConnection> = Mutex::new(establish_connection());
}

static MIGRATION_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub fn get_connection() -> std::sync::MutexGuard<'static, SqliteConnection> {
    DB_CONNECTION.lock().unwrap()
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_FILE").expect("DATABASE_FILE must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connectiong to {}", database_url))
}

pub fn create_db() {
    let cur_dir = env::current_dir().expect("Unable to get current directory");
    let db_filename = env::var("DATABASE_FILE").expect("DATABASE_FILE must be set");
    let db_path = cur_dir.join(db_filename);

    if db_path.exists() {
        // trace!("DB exists, skipping migrations");
        return;
    }

    // trace!("DB not exists, running migrations");

    run_migrations();
}

fn run_migrations() {
    let mut conn = get_connection();

    let mut subdirs: Vec<_> = MIGRATION_DIR.dirs().collect();
    subdirs.sort_by_key(|d| d.path());

    for subdir in subdirs {
        for mg_file in subdir.files() {
            match mg_file.path().file_name() {
                Some(name) => {
                    if "up.sql" != name {
                        continue;
                    }
                }
                None => continue,
            }

            let sql = match mg_file.contents_utf8() {
                Some(sql) => sql,
                None => panic!("Failed to read migration file: {:?}", mg_file.path().display()),
            };

            // sql_query only runs a single statement, so split the file on
            // statement-terminating semicolons and run each one in order.
            for statement in split_sql_statements(sql) {
                diesel::sql_query(statement)
                    .execute(&mut *conn)
                    .unwrap_or_else(|e| panic!(
                        "Failed to run migration {:?}: {}\nstatement: {}",
                        mg_file.path().display(), e, statement
                    ));
            }
        }
    }
}

fn split_sql_statements(sql: &str) -> Vec<&str> {
    sql.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect()
}
