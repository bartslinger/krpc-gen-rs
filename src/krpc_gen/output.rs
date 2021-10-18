use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
pub struct OutputStructure {
    pub methods: Vec<StandardMethod>,
    pub getters: Vec<PropertyGetterFunction>,
    pub setters: Vec<PropertySetterFunction>,
    pub classes: HashMap<String, Class>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Class {
    pub name: String,
    pub methods: Vec<StandardMethod>,
    pub getters: Vec<PropertyGetterFunction>,
    pub setters: Vec<PropertySetterFunction>,
    pub static_methods: Vec<StandardMethod>,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct PropertyGetterFunction {
    pub id: u64,
    pub procedure: String,
    pub name: String,
    pub decoder_function: String,
    pub return_type_signature: String,
    pub return_value: String,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct PropertySetterFunction {
    pub id: u64,
    pub procedure: String,
    pub name: String,
    pub decoder_function: String,
    pub return_type_signature: String,
    pub return_value: String,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct StandardMethod {
    pub id: u64,
    pub procedure: String,
    pub name: String,
    pub decoder_function: String,
    pub return_type_signature: String,
    pub return_value: String,
}
