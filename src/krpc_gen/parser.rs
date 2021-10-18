use std::collections::HashMap;
use regex::Regex;
use convert_case::{Case, Casing};
use crate::original;
use crate::output;

#[derive(PartialEq, Debug)]
struct StandardMethod {
    procedure: String,
    name: String,    
}

#[derive(PartialEq, Debug)]
struct ServiceProperty {
    procedure: String,
    name: String,
}

#[derive(PartialEq, Debug)]
struct ClassMethod {
    procedure: String,
    class: String,
    method: String,
}

#[derive(PartialEq, Debug)]
struct ClassProperty {
    procedure: String,
    class: String,
    property: String,
}

#[derive(PartialEq, Debug)]
enum ProcedureType {
    Standard(StandardMethod),
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
                    procedure: procedure_name.to_string(),
                    class: (&cap[1]).to_string(),
                    property: (&cap[3]).to_string(),
                });
            },
            "set" => {
                return ProcedureType::ClassPropertySetter(ClassProperty {
                    procedure: procedure_name.to_string(),
                    class: (&cap[1]).to_string(),
                    property: (&cap[3]).to_string(),
                });
            },
            "static" => {
                return ProcedureType::StaticClassMethod(ClassMethod {
                    procedure: procedure_name.to_string(),
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
                    procedure: procedure_name.to_string(),
                    name: (&cap[2]).to_string(),
                })
            },
            "set" => {
                return ProcedureType::PropertySetter(ServiceProperty {
                    procedure: procedure_name.to_string(),
                    name: (&cap[2]).to_string(),
                })
            },
            _ => {
                return ProcedureType::ClassMethod(ClassMethod {
                    procedure: procedure_name.to_string(),
                    class: (&cap[1]).to_string(),
                    method: (&cap[2]).to_string(),
                })
            }
        }
    }
    return ProcedureType::Standard(StandardMethod {
        procedure: procedure_name.to_string(),
        name: procedure_name.to_string(),
    })
}

pub fn create_output_structure(input_structure: &original::Content) -> output::OutputStructure {
    let mut service_methods = Vec::<output::StandardMethod>::new();
    let mut service_getters = Vec::<output::PropertyGetterFunction>::new();
    let mut service_setters = Vec::<output::PropertySetterFunction>::new();
    let mut classes = HashMap::<String, output::Class>::new();
    for proc in &input_structure.procedures {
        let procedure_type = get_procedure_type(proc.0);
        match &procedure_type {
            ProcedureType::Standard(x) => {
                service_methods.push(convert_service_method(&x, &proc.1));
            },
            ProcedureType::PropertyGetter(x) => {
                service_getters.push(convert_property_getter(&x, &proc.1));
            },
            ProcedureType::PropertySetter(x) => {
                service_setters.push(convert_property_setter(&x, &proc.1));
            },
            ProcedureType::ClassMethod(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().methods.push(convert_class_method(&x, &proc.1));
            },
            ProcedureType::ClassPropertyGetter(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().getters.push(convert_class_property_getter(&x, &proc.1));
            },
            ProcedureType::ClassPropertySetter(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().getters.push(convert_class_property_setter(&x, &proc.1));
            },
            ProcedureType::StaticClassMethod(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().methods.push(convert_static_class_method(&x, &proc.1));
            },
            ProcedureType::Unknown => {}
        }
    }
    
    // Sort lists
    service_methods.sort();
    service_getters.sort();
    service_setters.sort();
    
    for (_, class) in &mut classes {
        class.methods.sort();
        class.getters.sort();
        class.setters.sort();
        class.static_methods.sort();
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
            name: class_name.clone(),
            methods: vec![],
            getters: vec![],
            setters: vec![],
            static_methods: vec![],
        });
    }
}

fn convert_service_method(property: &StandardMethod, procedure: &original::Procedure) -> output::StandardMethod {
    output::StandardMethod {
        id: procedure.id,
        procedure: property.procedure.clone(),
        name: property.name.to_case(Case::Snake),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn convert_property_getter(property: &ServiceProperty, procedure: &original::Procedure) -> output::PropertyGetterFunction {
    output::PropertyGetterFunction {
        id: procedure.id,
        procedure: property.procedure.clone(),
        name: "get_".to_string() + property.name.to_case(Case::Snake).as_str(),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn convert_property_setter(property: &ServiceProperty, procedure: &original::Procedure) -> output::PropertySetterFunction {
    output::PropertySetterFunction {
        id: procedure.id,
        procedure: property.procedure.clone(),
        name: "set_".to_string() + property.name.to_case(Case::Snake).as_str(),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn convert_class_method(property: &ClassMethod, procedure: &original::Procedure) -> output::StandardMethod {
    output::StandardMethod {
        id: procedure.id,
        procedure: property.procedure.clone(),
        name: property.method.to_case(Case::Snake),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn convert_class_property_getter(property: &ClassProperty, procedure: &original::Procedure) -> output::PropertyGetterFunction {
    output::PropertyGetterFunction {
        id: procedure.id,
        procedure: property.procedure.clone(),
        name: "get_".to_string() + property.property.to_case(Case::Snake).as_str(),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn convert_class_property_setter(property: &ClassProperty, procedure: &original::Procedure) -> output::PropertyGetterFunction {
    output::PropertyGetterFunction {
        id: procedure.id,
        procedure: property.procedure.clone(),
        name: "set_".to_string() + property.property.to_case(Case::Snake).as_str(),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn convert_static_class_method(property: &ClassMethod, procedure: &original::Procedure) -> output::StandardMethod {
    output::StandardMethod {
        id: procedure.id,
        procedure: property.procedure.clone(),
        name: property.method.to_case(Case::Snake),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn return_type_signature(procedure: &original::Procedure) -> String {
    match &procedure.return_type {
        Some(return_type) => {
            match (&return_type.code, &return_type.name) {
                (original::Code::Class, Some(name)) => {
                    name.clone() + "<'a>"
                },
                _ => {
                    "()".to_string()
                }
            }
        },
        None => {
            "()".to_string()
        },
    }
}

fn return_value(procedure: &original::Procedure) -> String {
    match &procedure.return_type {
        Some(return_type) => {
            match (&return_type.code, &return_type.name) {
                (original::Code::Class, Some(name)) => {
                    format!("{}{{id: 0, conn: &self.conn}}", name)
                },
                _ => {
                    "()".to_string()
                }
            }
        },
        None => {
            "()".to_string()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_standard_method() {
        let result = get_procedure_type("WarpTo");
        let expected = ProcedureType::Standard(StandardMethod {
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