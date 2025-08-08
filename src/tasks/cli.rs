use crate::db::get_new_data::fetch_and_store_data;

pub async fn cli(args: Vec<String>) {
    // CLI
    let first_arg = &args[1];
    if first_arg == "install" {
        match fetch_and_store_data().await {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    } else {
        eprintln!("help: only legal command is 'install'");
    }
}
