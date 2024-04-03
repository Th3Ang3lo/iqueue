#[cfg(test)]
mod store_message_utils_test {
    use iqueue::utils::{file_utils::FileUtils, store_message_utils::StoreMessageUtils};

    #[test]
    fn store_message_in_file_success() -> Result<(), Box<dyn std::error::Error>> {
        let channel = "test_channel";

        let message_stored = StoreMessageUtils::store_message(channel, "content")?;
        
        let cwd = std::env::current_dir()?;

        let test_queue_file = format!(
            "{}/.iqueue/messages/{}.iqueue",
            cwd.to_string_lossy(),
            channel
        );

        let queue_file_deleted = FileUtils::delete_file(test_queue_file.as_str());

        assert!(message_stored);
        assert!(queue_file_deleted.is_some());

        Ok(())
    }
}
