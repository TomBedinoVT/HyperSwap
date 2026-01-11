use chrono::{DateTime, Utc, Duration};

pub fn add_days_to_now(days: u32) -> DateTime<Utc> {
    Utc::now() + Duration::days(days as i64)
}

pub fn is_expired(expires_at: Option<DateTime<Utc>>) -> bool {
    if let Some(exp) = expires_at {
        exp < Utc::now()
    } else {
        false
    }
}

