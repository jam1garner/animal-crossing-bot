use tokio::time::delay_for;
use chrono::{Duration, FixedOffset, Timelike, TimeZone, Utc, DateTime};

pub fn current_datetime_edt() -> DateTime<FixedOffset> {
    let edt = FixedOffset::west(4 * 60 * 60);
    edt.timestamp(Utc::now().timestamp(), 0)
}

pub fn time_to_wait_for() -> DateTime<FixedOffset> {
    let now = current_datetime_edt();
    let day_to_wait_for = if now.hour() > 9 {
        now + Duration::days(1)
    } else {
        now
    };
    
    day_to_wait_for
        .with_hour(9).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
}

pub async fn wait_till_next_day() {
    let wait_until = time_to_wait_for();

    println!("Waiting until {}...", wait_until);

    let duration = 
        wait_until
            .signed_duration_since(Utc::now())
            .to_std()
            .unwrap();

    println!("Waiting for {:?}...", duration);

    delay_for(duration).await;
}
