#[cfg(test)]
mod store_message_utils_test {
    use iqueue::utils::{file_utils::FileUtils, time_utils::TimeUtils};

    #[test]
    fn test_file_exists_success() {
        let cwd = std::env::current_dir().unwrap();

        let file = format!("{}/tests/files/test.txt", cwd.to_string_lossy());
        let file_exists = FileUtils::file_exists(file.as_str());

        assert!(file_exists.is_some());
    }

    #[test]
    fn test_read_file_as_string_success() {
        let cwd = std::env::current_dir().unwrap();

        let file = format!("{}/tests/files/test.txt", cwd.to_string_lossy());
        let file_content = FileUtils::read_file_as_string(file.as_str());

        assert!(file_content.is_ok());

        let content_as_string = file_content.unwrap();

        assert_eq!(content_as_string, "test");
    }

    #[test]
    fn test_append_file_success() {
        let current_timestamp = TimeUtils::get_current_unix_timestamp();

        assert!(current_timestamp.is_ok());

        let cwd = std::env::current_dir().unwrap();

        let current_timestamp = current_timestamp.unwrap();

        let file = format!(
            "{}/tests/files/{}_to_append.txt",
            cwd.to_string_lossy(),
            current_timestamp
        );

        let file_appended_result = FileUtils::append(file.as_str(), "test");

        assert!(file_appended_result.is_ok());

        let file_content_result = FileUtils::read_file_as_string(file.as_str());
        assert!(file_content_result.is_ok());

        let file_content = file_content_result.unwrap();
        assert_eq!(file_content.trim(), "test");

        let file_deleted_result = FileUtils::delete_file(file.as_str());
        assert!(file_deleted_result.is_ok());
    }

    #[test]
    fn test_remove_file_success() {
        let current_timestamp = TimeUtils::get_current_unix_timestamp();
        assert!(current_timestamp.is_ok());

        let cwd = std::env::current_dir().unwrap();

        let current_timestamp = current_timestamp.unwrap();

        let file = format!(
            "{}/tests/files/{}_to_delete.txt",
            cwd.to_string_lossy(),
            current_timestamp
        );

        let file_appended_result = FileUtils::append(file.as_str(), "test");
        assert!(file_appended_result.is_ok());

        let file_exists_result = FileUtils::file_exists(file.as_str());
        assert!(file_exists_result.is_some());

        let file_deleted_result = FileUtils::delete_file(file.as_str());
        assert!(file_deleted_result.is_ok());
    }
}
