use crate::tasks::import_new_data::import_new_data;

pub async fn cli(args: Vec<String>) {
    // CLI
    let first_arg = &args[1];
    if first_arg == "install" {
        if let Err(e) = import_new_data().await {
            eprintln!("install error: {}", e);
        };
    } else {
        eprintln!("help: only working command is 'install'");
    }
}
