use std::collections::HashMap;
use regex::Regex;
use convert_case::{Case, Casing};
use crate::original;
use crate::output;

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

pub fn create_output_structure(input_structure: &original::Content) -> output::OutputStructure {
    let mut service_methods = HashMap::<String, original::Procedure>::new();
    let mut service_getters = HashMap::<String, output::Function>::new();
    let mut service_setters = HashMap::<String, original::Procedure>::new();
    let mut classes = HashMap::<String, output::Class>::new();
    for proc in &input_structure.procedures {
        let procedure_type = get_procedure_type(proc.0);
        match &procedure_type {
            ProcedureType::Standard(x) => {
                service_methods.insert(x.name.clone(), (proc.1).clone());
            },
            ProcedureType::PropertyGetter(x) => {
                service_getters.insert(x.name.clone(), convert_to_function(&procedure_type, &proc.1));
            },
            ProcedureType::PropertySetter(x) => {
                service_setters.insert(x.name.clone(), (proc.1).clone());
            },
            ProcedureType::ClassMethod(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().methods.insert(x.method.clone(), (proc.1).clone());
            },
            ProcedureType::ClassPropertyGetter(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().getters.insert(x.property.clone(), (proc.1).clone());
            },
            ProcedureType::ClassPropertySetter(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().setters.insert(x.property.clone(), (proc.1).clone());
            },
            ProcedureType::StaticClassMethod(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().static_methods.insert(x.method.clone(), (proc.1).clone());
            },
            ProcedureType::Unknown => {}
        }
    }
    
    output::OutputStructure {
        methods: service_methods,
        getters: service_getters,
        setters: service_setters,
        classes: classes,
    }
}

fn add_class_if_nonexistent(classes: &mut HashMap<String, output::Class>, class_name: &String) {
    if let None = classes.get(class_name) {
        classes.insert(class_name.clone(), output::Class {
            methods: HashMap::new(),
            getters: HashMap::new(),
            setters: HashMap::new(),
            static_methods: HashMap::new(),
        });
    }

}

fn convert_to_function(procedure_type: &ProcedureType, procedure: &original::Procedure) -> output::Function {
    let return_type = match &procedure.return_type {
        Some(t) => {
            match t.code {
                original::Code::Class => {
                    output::ReturnType::Class{ name: t.name.clone().unwrap()}
                },
                _ => output::ReturnType::Empty
            }
        },
        None => output::ReturnType::Empty,
    };
    
    let function_name = match &procedure_type {
        ProcedureType::Standard(x) => x.name.to_case(Case::Snake),
        ProcedureType::PropertyGetter(x) => "get_".to_string() + x.name.to_case(Case::Snake).as_str(),
        ProcedureType::PropertySetter(x) => "set_".to_string() + x.name.to_case(Case::Snake).as_str(),
        ProcedureType::ClassMethod(x) => x.method.to_case(Case::Snake),
        ProcedureType::ClassPropertyGetter(x) => "get_".to_string() + x.class.to_case(Case::Snake).as_str(),
        ProcedureType::ClassPropertySetter(x) => "set_".to_string() + x.class.to_case(Case::Snake).as_str(),
        ProcedureType::StaticClassMethod(x) => x.method.to_case(Case::Snake),
        ProcedureType::Unknown => "".to_string(),
    };

    output::Function {
        name: function_name,
        return_type,
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