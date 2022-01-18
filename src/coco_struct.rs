use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberInfo {
    pub name: String,
    pub access: String,
    pub data_type: String,
    pub pure_data_type: String,
    pub line_no: i32
}

impl MemberInfo {
    pub fn new(name: &str, access: String, data_type: String) -> Self {
        MemberInfo {
            name: name.to_string(),
            access,
            data_type,
            pure_data_type: "".to_string(),
            line_no: 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub access: String,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub pure_return_type: String,
    pub line_no: i32,
}

impl MethodInfo {
    pub fn new(name: &str, access: String, parameters: Vec<String>, return_type: String) -> Self {
        MethodInfo {
            name: name.to_string(),
            access,
            parameters,
            return_type,
            pure_return_type: "".to_string(),
            line_no: 0
        }
    }

    pub fn parameter_too_long(&self) -> bool {
        self.parameters.len() > 5
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassInfo {
    pub id: i32,
    pub name: String,
    pub package: String,
    pub file: String,
    pub lang: String,
    pub parents: Vec<String>,
    pub members: Vec<MemberInfo>,
    pub methods: Vec<MethodInfo>,
}

impl ClassInfo {
    pub fn new(class_name: &str) -> Self {
        ClassInfo {
            id: 0,
            name: class_name.to_string(),
            package: "".to_string(),
            file: "".to_string(),
            lang: "".to_string(),
            parents: vec![],
            members: vec![],
            methods: vec![],
        }
    }
}
