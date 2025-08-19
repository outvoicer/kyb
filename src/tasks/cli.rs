use crate::tasks::import_new_data::import_new_data;

pub async fn cli(args: Vec<String>) {
    // CLI
    let first_arg = &args[1];
    if first_arg == "install" {
        import_new_data().await;
    } else {
        eprintln!("help: only legal command is 'install'");
    }
}
