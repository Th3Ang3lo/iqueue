#[cfg(test)]
mod store_message_utils_test {
    use iqueue::utils::{file_utils::{FileUtils, FileUtilsImpl}, store_message_utils::{StoreMessageUtils, StoreMessageUtilsImpl}};

    #[test]
    fn store_message_in_file_success() -> Result<(), Box<dyn std::error::Error>> {
        let channel = "test_channel";

        let message_stored = <StoreMessageUtils as StoreMessageUtilsImpl>::store_message(channel, "content")?;
        
        let cwd = std::env::current_dir()?;

        let test_queue_file = format!(
            "{}/.iqueue/messages/{}.iqueue",
            cwd.to_string_lossy(),
            channel
        );

        let queue_file_deleted = <FileUtils as FileUtilsImpl>::delete_file(test_queue_file.as_str());

        assert!(message_stored);
        assert!(queue_file_deleted.is_some());

        Ok(())
    }
}
