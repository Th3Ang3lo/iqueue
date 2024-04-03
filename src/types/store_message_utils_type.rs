use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageStoreFormat {
    pub timestamp: u64,
    pub data: String,
}
