use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    id: u64,
    pub procedures: HashMap<String, Procedure>,
    pub classes: HashMap<String, Class>,
    pub enumerations: HashMap<String, Enumeration>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Class {
    // documentation: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Enumeration {
    // documentation: String,
    pub values: Vec<EnumerationValue>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnumerationValue {
    pub name: String,
    pub value: u64,
    // documentation: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Procedure {
    pub id: u64,
    pub parameters: Vec<Parameter>,
    game_scenes: Option<Vec<GameScene>>,
    pub return_type: Option<ReturnType>,
    return_is_nullable: Option<bool>,
    // documentation: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub r#type: Type,
    pub default_value: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Type {
    pub code: Code,
    pub types: Option<Vec<Type>>,
    pub service: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReturnType {
    pub code: Code,
    pub types: Option<Vec<ReturnType>>,
    pub service: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Code {
    String,
    Bool,
    Float,
    Double,
    Sint32,
    Uint32,
    Enumeration,
    List,
    Dictionary,
    Set,
    Tuple,
    Class,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
enum GameScene {
    Flight,
}

pub fn deserialize_from_file(path: &std::path::Path) -> HashMap<String, Content> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let v: HashMap<String, Content> = serde_json::from_reader(reader).unwrap();

    v
}