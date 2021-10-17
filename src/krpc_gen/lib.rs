use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use serde::Deserialize;
use regex::Regex;

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
    return_type: Option<ReturnType>,
    return_is_nullable: Option<bool>,
    // documentation: String,
}

#[derive(Deserialize, Debug)]
struct Parameter {
    name: String,
    r#type: Type,
}

#[derive(Deserialize, Debug)]
struct Type {
    code: Code,
}

#[derive(Deserialize, Debug)]
struct ReturnType {
    code: Code,
    types: Option<Vec<Type>>,
    service: Option<String>,
    name: Option<String>,
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

// Self defined type

#[derive(PartialEq, Debug)]
struct ServiceMethod {
    name: String,    
}

#[derive(PartialEq, Debug)]
struct ServiceProperty {
    name: String,
}

#[derive(PartialEq, Debug)]
struct ClassMethod {
    class: String,
    method: String,
}

#[derive(PartialEq, Debug)]
struct ClassProperty {
    class: String,
    property: String,
}


#[derive(PartialEq, Debug)]
enum ProcedureType {
    Standard(ServiceMethod),
    PropertyGetter(ServiceProperty),
    PropertySetter(ServiceProperty),
    ClassMethod(ClassMethod),
    StaticClassMethod(ClassMethod),
    ClassPropertyGetter(ClassProperty),
    ClassPropertySetter(ClassProperty),
    Unknown,
}

fn get_procedure_type(procedure_name: &str) -> ProcedureType {
    // Patern with static/get/set in the middle
    let re = Regex::new(r"(.*)_(get|set|static)_(.*)").unwrap();
    let captures = re.captures(procedure_name);
    if let Some(cap) = captures {
        match &cap[2] {
            "get" => {
                return ProcedureType::ClassPropertyGetter(ClassProperty {
                    class: (&cap[1]).to_string(),
                    property: (&cap[3]).to_string(),
                });
            },
            "set" => {
                return ProcedureType::ClassPropertySetter(ClassProperty {
                    class: (&cap[1]).to_string(),
                    property: (&cap[3]).to_string(),
                });
            },
            "static" => {
                return ProcedureType::StaticClassMethod(ClassMethod {
                    class: (&cap[1]).to_string(),
                    method: (&cap[3]).to_string(),
                });
            },
            _ => {
                return ProcedureType::Unknown;
            }
        }
    }
    let re = Regex::new(r"(get|set|.*)_(.*)").unwrap();
    let captures = re.captures(procedure_name);
    if let Some(cap) = captures {
        match &cap[1] {
            "get" => {
                return ProcedureType::PropertyGetter(ServiceProperty {
                    name: (&cap[2]).to_string(),
                })
            },
            "set" => {
                return ProcedureType::PropertySetter(ServiceProperty {
                    name: (&cap[2]).to_string(),
                })
            },
            _ => {
                return ProcedureType::ClassMethod(ClassMethod {
                    class: (&cap[1]).to_string(),
                    method: (&cap[2]).to_string(),
                })
            }
        }
    }
    return ProcedureType::Standard(ServiceMethod {
        name: procedure_name.to_string(),
    })
}

pub fn generate_for(path: &str) {

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let v: FileStructure = serde_json::from_reader(reader).unwrap();

    for proc in &v.space_center.procedures {
        // println!("{:?}", proc.0);
        let procedure_type = get_procedure_type(proc.0);
        // println!("{:?}", procedure_type);
    }
    
    // println!("{:?}", v.space_center.procedures["get_ActiveVessel"]);
    
    // Convert into some more useable structure for code generation
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_standard_method() {
        let result = get_procedure_type("WarpTo");
        let expected = ProcedureType::Standard(ServiceMethod {
            name: "WarpTo".to_string(),
        });
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_service_getter() {
        let result = get_procedure_type("get_WarpMode");
        let expected = ProcedureType::PropertyGetter(ServiceProperty {
            name: "WarpMode".to_string(),
        });
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_class_method() {
        let result = get_procedure_type("AutoPilot_Engage");
        let expected = ProcedureType::ClassMethod(ClassMethod {
            class: "AutoPilot".to_string(),
            method: "Engage".to_string(),
        });
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_class_property_getter() {
        let result = get_procedure_type("Vessel_get_Type");
        let expected = ProcedureType::ClassPropertyGetter(ClassProperty {
            class: "Vessel".to_string(),
            property: "Type".to_string(),
        });
        assert_eq!(result, expected);
    }

}