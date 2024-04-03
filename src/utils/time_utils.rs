use std::time::{SystemTime, SystemTimeError};

pub struct TimeUtils;

impl TimeUtils {
    pub fn get_current_unix_timestamp() -> Result<u64, SystemTimeError> {
        let duration_since_unix_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        let current_duration = duration_since_unix_epoch.as_secs();

        Ok(current_duration)
    }
}
