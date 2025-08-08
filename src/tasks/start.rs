use crate::tasks::schedule::schedule_update;
use crate::tasks::server::start_server;
use actix_rt::spawn;

pub async fn start_kyb_server() {
    // SCHEDULE DB UPDATE
    spawn(schedule_update());
    // SPAWN SERVER IN A THREAD
    let server_handle = spawn(start_server());
    // AWAIT FOR SERVER
    match server_handle.await {
        Ok(_) => {
            println!("Server closed");
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    };
}
