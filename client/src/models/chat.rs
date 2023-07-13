use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Chat {
    pub openai_key: String,
}

// override default values with environment variables
impl Default for Chat {
    fn default() -> Self {
        Chat {
            openai_key: "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".to_string(),
        }
    }
}
