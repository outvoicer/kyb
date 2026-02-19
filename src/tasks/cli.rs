use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::tasks::import_new_data::import_new_data;

pub async fn cli(args: Vec<String>, pool: Pool<SqliteConnectionManager>) {
    // CLI
    let first_arg = &args[1];
    if first_arg == "install" {
        if let Err(e) = import_new_data(pool).await {
            eprintln!("install error: {}", e);
        };
    } else {
        eprintln!("help: only working command is 'install'");
    }
}
