use std::collections::HashMap;
use regex::Regex;
use crate::original;

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

pub struct OutputStructure {
    pub classes: HashMap<String, Class>,
}

#[derive(Debug)]
pub struct Class {
    pub methods: Vec<String>,
    pub getters: HashMap<String, original::Procedure>,
    pub setters: Vec<String>,
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

pub fn create_output_structure(input_structure: &original::FileStructure) -> OutputStructure {
    let mut classes = HashMap::<String, Class>::new();
    for proc in &input_structure.space_center.procedures {
        // println!("{:?}", proc.0);
        let procedure_type = get_procedure_type(proc.0);

        match procedure_type {
            ProcedureType::ClassMethod(x) => {
                match classes.get_mut(&x.class) {
                    Some(class) => {
                        class.methods.push(x.method);
                    },
                    None => {
                        classes.insert(x.class, Class {
                            methods: vec![x.method],
                            getters: HashMap::new(),
                            setters: vec![],
                        });
                    }
                }
            },
            ProcedureType::ClassPropertyGetter(x) => {
                match classes.get_mut(&x.class) {
                    Some(class) => {
                        class.getters.insert(x.property, (proc.1).clone());
                    },
                    None => {
                        let mut map = HashMap::new();
                        map.insert(x.property, (proc.1).clone());
                        classes.insert(x.class, Class {
                            methods: vec![],
                            getters: map,
                            setters: vec![],
                        });
                    }
                }
            },
            ProcedureType::ClassPropertySetter(x) => {
                match classes.get_mut(&x.class) {
                    Some(class) => {
                        class.setters.push(x.property);
                    },
                    None => {
                        classes.insert(x.class, Class {
                            methods: vec![],
                            getters: HashMap::new(),
                            setters: vec![x.property],
                        });
                    }
                }
            },
            _ => {}
        }
    }
    
    OutputStructure {
        classes: classes,
    }
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