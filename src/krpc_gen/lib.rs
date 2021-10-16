use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FileStructure {
    #[serde(rename = "SpaceCenter")]
    space_center: Content,
}

#[derive(Deserialize, Debug)]
struct Content {
    id: u64,
    procedures: HashMap<String, Procedure>,
}

#[derive(Deserialize, Debug)]
struct Procedure {
    id: u64,
    parameters: Vec<Parameter>,
    game_scenes: Option<Vec<GameScene>>,
    return_type: Option<Type>,
}

#[derive(Deserialize, Debug)]
struct Parameter {
    name: String,
    r#type: Type,
}

#[derive(Deserialize, Debug)]
struct Type {
    code: Code,
    types: Option<Vec<Type>>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum GameScene {
    Flight,
}

pub fn generate_for(path: &str) {

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let v: FileStructure = serde_json::from_reader(reader).unwrap();

    for proc in v.space_center.procedures {
        println!("{:?}", proc.0);
    }
}