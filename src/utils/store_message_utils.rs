use std::io::{Error, ErrorKind};

use crate::types::store_message_utils_type::MessageStoreFormat;
use crate::utils::{file_utils::FileUtils, time_utils::TimeUtils};

pub struct StoreMessageUtils;

impl StoreMessageUtils {
    pub fn store_message(channel: &str, data: &str) -> Result<bool, Error> {
        let cwd = std::env::current_dir()?;

        let queue_log_file_path = format!(
            "{}/.iqueue/messages/{}.iqueue",
            cwd.to_string_lossy(),
            channel
        );

        let current_timestamp_result = TimeUtils::get_current_unix_timestamp();

        if let Err(err) = current_timestamp_result {
            panic!("Error getting current timestamp: {}", err);
        }

        let current_timestamp = current_timestamp_result.unwrap();

        let data = MessageStoreFormat {
            timestamp: current_timestamp,
            data: data.to_string(),
        };

        let queue_string_result = serde_json::to_string(&data)?;
        let append_on_queue = FileUtils::append(
            queue_log_file_path.as_str(),
            queue_string_result.as_str(),
        )?;

        if append_on_queue {
            return Ok(true);
        }

        Err(Error::new(
            ErrorKind::Other,
            "Error on store queue data on file.",
        ))
    }
}
