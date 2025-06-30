use std::sync::{Arc, Mutex};

use miette::{Result, miette};
use tokio::net::TcpListener;

use gURL::{db::db, env::env, init_router};

#[tokio::main]
async fn main() -> Result<()> {
    let (db, port) = (db(true, false)?, env()?.port);
    let global_db = Arc::new(Mutex::new(db));
    let router = init_router(global_db);

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .map_err(|e| miette!("Tokio unable to listen on port {port}: {e}"))?;

    axum::serve(listener, router)
        .await
        .map_err(|e| miette!("Axum unable to serve gURL router: {e}"))?;

    Ok(())
}
