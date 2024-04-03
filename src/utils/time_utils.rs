use core::panic;
use std::time::{SystemTime, SystemTimeError};

pub struct TimeUtils;

impl TimeUtils {
    pub fn get_current_unix_timestamp() -> Result<u64, SystemTimeError> {
        let duration_since_unix_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

        if let Err(err) = duration_since_unix_epoch {
            panic!("Error getting current time: {}", err);
        }

        let current_duration = duration_since_unix_epoch.unwrap().as_secs();

        return Ok(current_duration);
    }
}
