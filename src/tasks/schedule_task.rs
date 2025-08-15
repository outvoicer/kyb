use crate::error::KybError;
use chrono::{Local, Timelike};
use tokio::time::{Duration, Instant, sleep_until};

pub async fn wait_for_task(hour: u32, minute: u32) -> Result<(), KybError> {
    let now = Local::now();
    let next_run = now
        .with_hour(hour)
        .unwrap()
        .with_minute(minute)
        .unwrap()
        .with_second(0)
        .unwrap();

    let duration_until_next_run = if now < next_run {
        next_run - now
    } else {
        next_run + chrono::Duration::days(1) - now
    };

    sleep_until(Instant::now() + Duration::from_secs(duration_until_next_run.num_seconds() as u64))
        .await;
    Ok(())
}
