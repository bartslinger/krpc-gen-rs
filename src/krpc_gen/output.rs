use std::collections::HashMap;
use serde::Serialize;
use crate::original;

pub struct OutputStructure {
    pub methods: Vec<StandardMethod>,
    pub getters: Vec<PropertyGetterFunction>,
    pub setters: Vec<PropertySetterFunction>,
    pub classes: HashMap<String, Class>,
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub methods: Vec<StandardMethod>,
    pub getters: Vec<PropertyGetterFunction>,
    pub setters: Vec<PropertySetterFunction>,
    pub static_methods: Vec<StandardMethod>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Function {
    pub name: String,
    pub return_type_signature: String,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct PropertyGetterFunction {
    pub id: u64,
    pub name: String,
    pub return_type_signature: String,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct PropertySetterFunction {
    pub id: u64,
    pub name: String,
    pub return_type_signature: String,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct StandardMethod {
    pub id: u64,
    pub name: String,
    pub return_type_signature: String,
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