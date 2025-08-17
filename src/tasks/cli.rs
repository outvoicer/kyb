use crate::company::get_new_company_data::fetch_new_company_data;
use crate::db::get_new_data::fetch_and_store_data;

pub async fn cli(args: Vec<String>) {
    // CLI
    let first_arg = &args[1];
    if first_arg == "install" {
        if let Err(e) = fetch_and_store_data().await {
            eprintln!("Error with member of board data: {}", e);
        }
        if let Err(e) = fetch_new_company_data().await {
            eprintln!("Error with company data: {}", e);
        }
    } else {
        eprintln!("help: only legal command is 'install'");
    }
}
