use std::fs;

use miette::{Result, miette};
use rusqlite::Connection;

use crate::utils::env::env;

pub fn db(init: bool, test: bool) -> Result<Connection> {
    let connection = if test {
        Connection::open_in_memory()
            .map_err(|e| miette!("sqlite connection could not be opened: {e}"))?
    } else {
        let env = env()?;

        if cfg!(feature = "docker") {
            Connection::open(format!("/.docker-db/{}", env.db_name))
                .map_err(|e| miette!("sqlite connection could not be opened: {e}"))?
        } else {
            Connection::open(env.db_name)
                .map_err(|e| miette!("sqlite connection could not be opened: {e}"))?
        }
    };

    if init {
        let db_path = match connection.path() {
            Some("") => "in memory".to_string(),
            Some(path) => format!("at {path}"),
            None => panic!("sqlite connection path is non-existent"),
        };

        println!("sqlite database file located {db_path}");

        let db_initialized = connection
            .table_exists(None, "request")
            .map_err(|e| miette!("Could not query request table{e}"))?;

        if !db_initialized {
            let init_script = fs::read_to_string("init.sql")
                .map_err(|e| miette!("init.sql could not be read: {e}"))?;

            connection
                .execute_batch(&init_script)
                .map_err(|e| miette!("Could not initialize db with init script: {e}"))?;
        }
    }

    Ok(connection)
}
