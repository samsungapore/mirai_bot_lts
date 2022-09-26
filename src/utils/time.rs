use chrono::{Datelike, Timelike};
use serenity::model::Timestamp;
use crate::bot::BOT_TIMEZONE;
use crate::log::{MiraiLog, MiraiLogger};
pub const FRENCH_TIME_FORMAT: &str = "%d/%m/%Y à %Hh%Mm%Ss";
pub const UTC_OFFSET: u32 = 2;

pub async fn wait_until_year_reached(year: i32, running: &bool, time_limit:
&Option<tokio::time::Instant>) ->
                                     Result<bool, ()> {
    let start = tokio::time::Instant::now();
    let period = chrono::Duration::days(1).to_std().unwrap();
    let mut interval = tokio::time::interval_at(start, period);

    let mut now: tokio::time::Instant;
    while *running {
        now = tokio::time::Instant::now();
        if time_limit.is_some() && now > time_limit.unwrap() {
            break;
        }

        if local_timestamp_now().date().year() == year {
            MiraiLogger::debug("Found year interval".to_string());
            return Ok(true);
        }

        interval.tick().await;
    }

    Ok(false)
}

pub async fn wait_until_day_reached(day: u32, running: &bool, time_limit:
&Option<tokio::time::Instant>) ->
                                    Result<bool, ()> {
    let start = tokio::time::Instant::now();
    let period = chrono::Duration::days(1).to_std().unwrap();
    let mut interval = tokio::time::interval_at(start, period);

    let mut now: tokio::time::Instant;
    while *running {
        now = tokio::time::Instant::now();
        if time_limit.is_some() && now > time_limit.unwrap() {
            break;
        }

        if local_timestamp_now().day() == day {
            MiraiLogger::debug("Found day interval".to_string());
            return Ok(true);
        }

        interval.tick().await;
    }

    Ok(false)
}

pub async fn wait_until_hour_reached(hour: u32, running: &bool, time_limit:
&Option<tokio::time::Instant>) ->
                                     Result<bool, ()> {
    let start = tokio::time::Instant::now();
    let period = chrono::Duration::hours(1).to_std().unwrap();
    let mut interval = tokio::time::interval_at(start, period);

    let mut now: tokio::time::Instant;
    while *running {
        now = tokio::time::Instant::now();
        if time_limit.is_some() && now > time_limit.unwrap() {
            break;
        }

        if local_timestamp_now().time().hour() + UTC_OFFSET == hour {
            MiraiLogger::debug("Found hour interval".to_string());
            return Ok(true);
        }

        interval.tick().await;
    }

    Ok(false)
}

pub async fn wait_until_minute_reached(minute: u32, running: &bool, time_limit:
&Option<tokio::time::Instant>) -> Result<bool, ()> {
    let start = tokio::time::Instant::now();
    let period = chrono::Duration::minutes(1).to_std().unwrap();
    let mut interval = tokio::time::interval_at(start, period);

    let mut now: tokio::time::Instant;
    while *running {
        now = tokio::time::Instant::now();
        if time_limit.is_some() && now > time_limit.unwrap() {
            break;
        }

        if local_timestamp_now().time().minute() == minute {
            MiraiLogger::debug("Found minute interval".to_string());
            return Ok(true);
        }

        interval.tick().await;
    }

    Ok(false)
}

pub async fn calibrate_seconds(running: &bool, time_limit: &Option<tokio::time::Instant>) ->
Result<bool, ()> {
    let start = tokio::time::Instant::now();
    let second_period = chrono::Duration::seconds(1).to_std().unwrap();
    let mut interval = tokio::time::interval_at(start, second_period);

    let mut now: tokio::time::Instant;
    let mut now_timestamp: Timestamp;
    while *running {
        now = tokio::time::Instant::now();
        if time_limit.is_some() && now > time_limit.unwrap() {
            break;
        }

        MiraiLogger::debug(format!("Tick."));

        now_timestamp = local_timestamp_now();
        if now_timestamp.time().second() == 0 {
            MiraiLogger::debug(format!("Seconds calibrated. {}", now_timestamp));
            return Ok(true);
        }

        interval.tick().await;
    }

    Ok(false)
}

pub async fn sync_at(
    running: &bool,
    time_limit: &Option<tokio::time::Instant>,
    timestamp: Timestamp,
    interval: Option<std::time::Duration>,
) -> Result<tokio::time::Interval, &'static str> {
    if !calibrate_seconds(running, time_limit).await.unwrap_or(false) {
        return Err("Could not calibrate seconds");
    }

    if !wait_until_minute_reached(timestamp.minute(), running, time_limit).await.unwrap_or
    (false) {
        return Err("Could not find minute");
    }

    if !wait_until_hour_reached(timestamp.hour(), running, time_limit).await.unwrap_or(false) {
        return Err("Could not find hour");
    }

    if !wait_until_day_reached(timestamp.day(), running, time_limit).await.unwrap_or(false) {
        return Err("Could not find day");
    }

    if !wait_until_year_reached(timestamp.year(), running, time_limit).await.unwrap_or(false) {
        return Err("Could not find year");
    }

    if let Some(interval_duration) = interval {
        let mut inter = tokio::time::interval(interval_duration);
        inter.tick().await;
        MiraiLogger::debug(format!("momentum found."));
        Ok(inter)
    } else {
        Ok(tokio::time::interval(std::time::Duration::new(0, 0)))
    }
}

pub fn local_timestamp_now() -> Timestamp {
    let t = Timestamp::now().with_timezone(&BOT_TIMEZONE);
    return Timestamp::from(t);
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use crate::utils::time::{calibrate_seconds, FRENCH_TIME_FORMAT, local_timestamp_now};

    #[tokio::test]
    async fn test_local_timestamp() {
        println!("{}", local_timestamp_now());
        println!("{}", local_timestamp_now().format(FRENCH_TIME_FORMAT));
        println!("HOUR:{}", local_timestamp_now().hour());
    }

    #[tokio::test]
    async fn test_seconds_calibration() {
        let running = true;
        let time_limit = Some(tokio::time::Instant::now() + tokio::time::Duration::new(70, 0));

        assert_eq!(calibrate_seconds(&running, &time_limit).await.unwrap_or(false), true);
    }
}