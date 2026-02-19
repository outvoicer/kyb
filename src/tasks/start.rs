use crate::server::server::start_server;
use crate::tasks::schedule::schedule_update;
use actix_rt::spawn;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub async fn start_kyb_server(pool: Pool<SqliteConnectionManager>) {
    // SCHEDULE DB UPDATE
    spawn(schedule_update(pool.clone()));
    // SPAWN SERVER IN A THREAD
    let server_handle = spawn(start_server(pool));
    // AWAIT FOR SERVER
    match server_handle.await {
        Ok(message) => {
            println!("Server closed {:?}", message);
        }
        Err(err) => {
            eprintln!("Server error: {:?}", err);
        }
    };
}
