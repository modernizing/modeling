use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParseOption {
    pub merge: bool,
    pub field_only: bool,
    pub without_parent: bool
}

impl Default for ParseOption {
    fn default() -> Self {
        ParseOption {
            merge: false,
            field_only: false,
            without_parent: false
        }
    }
}
