use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
pub struct OutputStructure {
    pub methods: Vec<Method>,
    pub getters: Vec<Method>,
    pub setters: Vec<Method>,
    pub classes: HashMap<String, Class>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
    pub getters: Vec<Method>,
    pub setters: Vec<Method>,
    pub static_methods: Vec<Method>,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Method {
    pub id: u64,
    pub procedure: String,
    pub name: String,
    pub decoder_function: String,
    pub return_type_signature: String,
    pub return_value: String,
}
