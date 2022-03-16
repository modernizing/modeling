use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ParseOption {
    pub merge_method_name: bool,
    pub field_only: bool,
    pub without_parent: bool,
    pub without_impl_suffix: bool,
    pub inline_id_suffix: bool,
    pub without_suffix: String,
}
