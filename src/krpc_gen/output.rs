use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
pub struct OutputStructure {
    pub methods: Vec<Method>,
    pub getters_setters: Vec<Method>,
    pub classes: HashMap<String, Class>,
    pub enumerations: Vec<Enumeration>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
    pub getters_setters: Vec<Method>,
    pub static_methods: Vec<Method>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Enumeration {
    pub name: String,
    pub values: Vec<EnumerationValue>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct EnumerationValue {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Argument {
    pub position: u64,
    pub encoder_function: String,
    pub value: String,
    pub optional: bool,
    pub name: String,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Method {
    pub id: u64,
    pub procedure: String,
    pub name: String,
    pub arguments_signature: String,
    pub arguments: Vec<Argument>,
    pub decoder_function: String,
    pub before_return: String,
    pub return_type_signature: String,
    pub return_value: String,
}
