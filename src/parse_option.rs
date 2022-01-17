use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParseOption {
    pub merge: bool,
    pub field_only: bool,
    pub without_parent: bool,
    pub remove_impl_suffix: bool,
    pub inline_id_suffix: bool,
}

impl Default for ParseOption {
    fn default() -> Self {
        ParseOption {
            merge: false,
            field_only: false,
            without_parent: false,
            remove_impl_suffix: false,
            inline_id_suffix: false
        }
    }
}
