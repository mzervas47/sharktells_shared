use std::time::{SystemTime, UNIX_EPOCH};

#[inline]
pub fn now_ms_utc() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis() as u64
}
