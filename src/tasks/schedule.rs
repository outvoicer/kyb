use crate::company::get_new_company_data::fetch_new_company_data;
use crate::config::KybConfig;
use crate::db::get_new_data::fetch_and_store_data;
use crate::tasks::schedule_task::wait_for_task;

pub async fn schedule_update() {
    loop {
        let hour = KybConfig::UPDATE_HOUR;
        let minute = KybConfig::UPDATE_MINUTE;
        let _ = wait_for_task(hour, minute).await;
        if let Err(e) = fetch_and_store_data().await {
            eprintln!("Error with member of board data: {}", e);
        }
        if let Err(e) = fetch_new_company_data().await {
            eprintln!("Error with company data: {}", e);
        }
    }
}
