mod config;
mod db;
mod error;
mod latvia;
mod tasks;
mod verify;

use crate::tasks::cli::cli;
use crate::tasks::start::start_kyb_server;
use std::env;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // CLI
        cli(args).await;
    } else {
        // NORMAL PROCEDURE
        start_kyb_server().await;
    }
}
