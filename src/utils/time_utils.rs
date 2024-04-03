use std::time::{SystemTime, SystemTimeError};

pub struct TimeUtils;

impl TimeUtils {
    pub fn get_current_unix_timestamp() -> Result<u64, SystemTimeError> {
        let duration_since_unix_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        let current_duration = match duration_since_unix_epoch {
            Ok(duration_since_unix_epoch) => Ok(duration_since_unix_epoch.as_secs()),
            Err(e) => Err(e)
        };

        return current_duration;
    }
}