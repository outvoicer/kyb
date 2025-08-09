use crate::config::KybConfig;
use crate::db::get_new_data::fetch_and_store_data;
use chrono::{Local, Timelike};
use tokio::time::{Duration, Instant, sleep_until};

pub async fn schedule_update() {
    loop {
        let now = Local::now();
        let next_run = now
            .with_hour(KybConfig::UPDATE_HOUR)
            .unwrap()
            .with_minute(KybConfig::UPDATE_MINUTE)
            .unwrap()
            .with_second(0)
            .unwrap();

        let duration_until_next_run = if now < next_run {
            next_run - now
        } else {
            next_run + chrono::Duration::days(1) - now
        };

        sleep_until(
            Instant::now() + Duration::from_secs(duration_until_next_run.num_seconds() as u64),
        )
        .await;
        if let Err(e) = fetch_and_store_data().await {
            eprintln!("Error fetching and storing data: {}", e);
        }
    }
}
