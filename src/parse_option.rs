use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParseOption {
    pub merge_method: bool,
}

impl Default for ParseOption {
    fn default() -> Self {
        ParseOption {
            merge_method: false
        }
    }
}
