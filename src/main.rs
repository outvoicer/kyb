mod config;
mod db;
mod error;
mod latvia;
mod server;
mod tasks;

use crate::db::get_db::get_db;
use crate::tasks::cli::cli;
use crate::tasks::start::start_kyb_server;
use std::env;

#[actix_web::main]
async fn main() {
    // GET DB
    match get_db() {
        Ok(db) => {
            // GET ARGS
            let args: Vec<String> = env::args().collect();
            if args.len() > 1 {
                // CLI
                cli(args, db).await;
            } else {
                // NORMAL PROCEDURE
                start_kyb_server(db).await;
            }
        }
        Err(err) => {
            eprintln!("db error: {}", err)
        }
    }
}
