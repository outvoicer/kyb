use crate::config::KybConfig;
use crate::tasks::import_new_data::import_new_data;
use chrono::{Duration as ChronoDuration, Local, Timelike};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::time::{Duration, Instant, sleep_until};

pub async fn schedule_update(pool: Pool<SqliteConnectionManager>) {
    let hour = KybConfig::UPDATE_HOUR;
    let minute = KybConfig::UPDATE_MINUTE;
    loop {
        let now = Local::now();
        // FIND NEXT RUN TIME
        let next_run = now
            .with_hour(hour)
            .unwrap()
            .with_minute(minute)
            .unwrap()
            .with_second(0)
            .unwrap();

        // CALCULATE TIME UNTIL NEXT RUN
        let duration_until_next_run = if now < next_run {
            next_run - now
        } else {
            next_run + ChronoDuration::days(1) - now
        };
        // WAIT UNTIL NEXT RUN
        sleep_until(
            Instant::now() + Duration::from_secs(duration_until_next_run.num_seconds() as u64),
        )
        .await;

        let _ = import_new_data(pool.clone()).await;
    }
}
