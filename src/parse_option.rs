use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParseOption {
    pub merge: bool,
}

impl Default for ParseOption {
    fn default() -> Self {
        ParseOption {
            merge: false
        }
    }
}
