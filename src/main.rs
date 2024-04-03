mod types;
mod utils;

use utils::store_message_utils::StoreMessageUtils;

fn main() {
    StoreMessageUtils::store_message("test_channel", "test");
}
