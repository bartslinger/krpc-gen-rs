use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FileStructure {
    #[serde(rename = "SpaceCenter")]
    pub space_center: Content,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    id: u64,
    pub procedures: HashMap<String, Procedure>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Procedure {
    id: u64,
    parameters: Vec<Parameter>,
    game_scenes: Option<Vec<GameScene>>,
    return_type: Option<ReturnType>,
    return_is_nullable: Option<bool>,
    // documentation: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Parameter {
    name: String,
    r#type: Type,
}

#[derive(Deserialize, Debug, Clone)]
struct Type {
    code: Code,
}

#[derive(Deserialize, Debug, Clone)]
struct ReturnType {
    code: Code,
    types: Option<Vec<Type>>,
    service: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
enum Code {
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

pub fn deserialize_from_file(path: &str) -> FileStructure {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let v: FileStructure = serde_json::from_reader(reader).unwrap();

    v
}