use std::fs;

use miette::{Result, miette};
use rusqlite::Connection;

use crate::utils::env::env;

pub fn db(init: bool) -> Result<Connection> {
    let env = env()?;
    let connection = Connection::open(env.db_path)
        .map_err(|e| miette!("sqlite connection could not be opened: {e}"))?;

    if init {
        println!("sqlite database file located at {:?}", connection.path());
        let db_empty = connection
            .table_exists(Some("gURL"), "request")
            .map_err(|e| miette!("Could not query request table{e}"))?;

        if db_empty {
            let init_script = fs::read_to_string("init.sql")
                .map_err(|e| miette!("init.sql could not be read: {e}"))?;

            connection
                .execute_batch(&init_script)
                .map_err(|e| miette!("Could not initialize db with init script: {e}"))?;
        }
    }

    Ok(connection)
}
