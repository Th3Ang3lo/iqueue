#[cfg(test)]
mod store_message_utils_test {
    use iqueue::utils::time_utils::TimeUtils;

    #[test]
    fn test_get_current_unix_timestamp_success() {
        let current_timestamp = TimeUtils::get_current_unix_timestamp();

        assert!(current_timestamp.is_ok());
        assert!(current_timestamp.unwrap() > 0);
    }
}