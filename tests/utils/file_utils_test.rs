#[cfg(test)]
mod store_message_utils_test {
    use iqueue::utils::{file_utils::{FileUtils, FileUtilsImpl}, time_utils::TimeUtils};

    #[test]
    fn test_file_exists_success() -> Result<(), Box<dyn std::error::Error>> {
        let cwd = std::env::current_dir()?;

        let file = format!("{}/tests/files/test.txt", cwd.to_string_lossy());
        let file_exists = <FileUtils as FileUtilsImpl>::file_exists(file.as_str());

        assert!(file_exists.is_some());
        assert!(file_exists.unwrap());

        Ok(())
    }

    #[test]
    fn test_read_file_as_string_success() -> Result<(), Box<dyn std::error::Error>> {
        let cwd = std::env::current_dir()?;

        let file_path = format!("{}/tests/files/test.txt", cwd.to_string_lossy());
        let file_content = <FileUtils as FileUtilsImpl>::read_file_as_string(file_path.as_str())?;

        assert_eq!(file_content, "test");
        
        Ok(())
    }

    #[test]
    fn test_append_file_success() -> Result<(), Box<dyn std::error::Error>> {
        let current_timestamp = TimeUtils::get_current_unix_timestamp()?;

        let cwd = std::env::current_dir()?;

        let file_path = format!(
            "{}/tests/files/{}_to_append.txt",
            cwd.to_string_lossy(),
            current_timestamp
        );

        let file_appended = <FileUtils as FileUtilsImpl>::append(file_path.as_str(), "test")?;
        let file_content = <FileUtils as FileUtilsImpl>::read_file_as_string(file_path.as_str())?;

        let _ = <FileUtils as FileUtilsImpl>::delete_file(file_path.as_str());

        assert!(file_appended);
        assert_eq!(file_content.trim(), "test");

        Ok(())
    }

    #[test]
    fn test_remove_file_success() -> Result<(), Box<dyn std::error::Error>> {
        let current_timestamp = TimeUtils::get_current_unix_timestamp()?;

        let cwd = std::env::current_dir()?;

        let file_path = format!(
            "{}/tests/files/{}_to_delete.txt",
            cwd.to_string_lossy(),
            current_timestamp
        );

        let file_appended = <FileUtils as FileUtilsImpl>::append(file_path.as_str(), "test")?;

        let file_exists_result = <FileUtils as FileUtilsImpl>::file_exists(file_path.as_str());
        

        <FileUtils as FileUtilsImpl>::delete_file(file_path.as_str());

        assert!(file_appended);
        assert!(file_exists_result.is_some());

        Ok(())
    }
}
