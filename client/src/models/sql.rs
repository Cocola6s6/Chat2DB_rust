use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Sql {
    pub url: String,
    pub ns: String, 
}

// override default values with environment variables
impl Default for Sql {
    fn default() -> Sql {
        Sql {
            url: "postgres://postgres:postgres@45.128.222.100:15432".to_string(),
            ns: "public".to_string(),
        }
    }
}
