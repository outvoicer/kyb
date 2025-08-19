use crate::config::KybConfig;
use crate::tasks::import_new_data::import_new_data;
use crate::tasks::schedule_task::wait_for_task;

pub async fn schedule_update() {
    loop {
        let hour = KybConfig::UPDATE_HOUR;
        let minute = KybConfig::UPDATE_MINUTE;
        let _ = wait_for_task(hour, minute).await;
        let _ = import_new_data();
    }
}
