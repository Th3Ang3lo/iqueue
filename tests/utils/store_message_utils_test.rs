#[cfg(test)]
mod store_message_utils_test {
    use iqueue::utils::{file_utils::FileUtils, store_message_utils::StoreMessageUtils};

    #[test]
    fn store_message_in_file_success() {
        let channel = "test_channel";

        let store_message_result = StoreMessageUtils::store_message(channel, "content");

        assert!(store_message_result.is_ok());

        let cwd = std::env::current_dir().unwrap();

        let test_queue_file = format!(
            "{}/.iqueue/messages/{}.iqueue",
            cwd.to_string_lossy(),
            channel
        );

        let delete_queue_file = FileUtils::delete_file(test_queue_file.as_str());
        assert!(delete_queue_file.is_ok());
    }
}
