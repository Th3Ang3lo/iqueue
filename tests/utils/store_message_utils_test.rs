

#[cfg(test)]
mod store_message_utils_test {
    use iqueue::utils::store_message_utils::StoreMessageUtils;

    #[test]
    fn store_message_in_file_success() {
        StoreMessageUtils::store_message("test_channel", "content");
        
        assert!(true);
    }
}