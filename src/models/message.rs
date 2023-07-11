use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub code: u32,
    pub result: String,
}