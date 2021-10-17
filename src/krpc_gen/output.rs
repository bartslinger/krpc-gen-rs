use std::collections::HashMap;
use serde::Serialize;
use crate::original;

pub struct OutputStructure {
    pub methods: HashMap<String, original::Procedure>,
    pub getters: HashMap<String, Function>,
    pub setters: HashMap<String, original::Procedure>,
    pub classes: HashMap<String, Class>,
}

#[derive(Debug)]
pub struct Class {
    pub methods: HashMap<String, original::Procedure>,
    pub getters: HashMap<String, original::Procedure>,
    pub setters: HashMap<String, original::Procedure>,
    pub static_methods: HashMap<String, original::Procedure>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Function {
    pub name: String,
    pub return_type: ReturnType,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ReturnType {
    Empty,
    Class {name: String},
    // Float,
    // Double,
    // Tuple,
}
impl Default for ReturnType {
    fn default() -> ReturnType {
        ReturnType::Empty
    }
}