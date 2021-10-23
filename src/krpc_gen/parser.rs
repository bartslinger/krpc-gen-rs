use std::collections::HashMap;
use regex::Regex;
use convert_case::{Case, Casing};
use crate::original;
use crate::output;

trait ParsedMethod {
    fn original_procedure_name(&self) -> String;
    fn function_name(&self) -> String;
}

#[derive(PartialEq, Debug)]
struct StandardMethod {
    procedure: String,
    name: String,    
}
impl ParsedMethod for StandardMethod {
    fn original_procedure_name(&self) -> String {
        self.procedure.clone()
    }
    fn function_name(&self) -> String {
        self.name.to_case(Case::Snake)
    }
}

#[derive(PartialEq, Debug)]
struct ServiceProperty {
    procedure: String,
    name: String,
    prefix: String,
}
impl ParsedMethod for ServiceProperty {
    fn original_procedure_name(&self) -> String {
        self.procedure.clone()
    }
    fn function_name(&self) -> String {
        self.prefix.clone() + self.name.to_case(Case::Snake).as_str()
    }
}

#[derive(PartialEq, Debug)]
struct ClassMethod {
    procedure: String,
    class: String,
    method: String,
}
impl ParsedMethod for ClassMethod {
    fn original_procedure_name(&self) -> String {
        self.procedure.clone()
    }
    fn function_name(&self) -> String {
        self.method.to_case(Case::Snake)
    }
}

#[derive(PartialEq, Debug)]
struct ClassProperty {
    procedure: String,
    class: String,
    property: String,
    prefix: String,
}
impl ParsedMethod for ClassProperty {
    fn original_procedure_name(&self) -> String {
        self.procedure.clone()
    }
    fn function_name(&self) -> String {
        self.prefix.clone() + self.property.to_case(Case::Snake).as_str()
    }
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
                    prefix: "get_".to_string(),
                });
            },
            "set" => {
                return ProcedureType::ClassPropertySetter(ClassProperty {
                    procedure: procedure_name.to_string(),
                    class: (&cap[1]).to_string(),
                    property: (&cap[3]).to_string(),
                    prefix: "set_".to_string(),
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
                    prefix: "get_".to_string(),
                })
            },
            "set" => {
                return ProcedureType::PropertySetter(ServiceProperty {
                    procedure: procedure_name.to_string(),
                    name: (&cap[2]).to_string(),
                    prefix: "set_".to_string(),
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
    let mut service_methods = Vec::<output::Method>::new();
    let mut service_getters = Vec::<output::Method>::new();
    let mut service_setters = Vec::<output::Method>::new();
    let mut classes = HashMap::<String, output::Class>::new();
    for proc in &input_structure.procedures {
        let procedure_type = get_procedure_type(proc.0);
        match &procedure_type {
            ProcedureType::Standard(x) => {
                service_methods.push(convert_method(x, &proc.1));
            },
            ProcedureType::PropertyGetter(x) => {
                service_getters.push(convert_method(x, &proc.1));
            },
            ProcedureType::PropertySetter(x) => {
                service_setters.push(convert_method(x, &proc.1));
            },
            ProcedureType::ClassMethod(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().methods.push(convert_method(x, &proc.1));
            },
            ProcedureType::ClassPropertyGetter(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().getters.push(convert_method(x, &proc.1));
            },
            ProcedureType::ClassPropertySetter(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().getters.push(convert_method(x, &proc.1));
            },
            ProcedureType::StaticClassMethod(x) => {
                add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().methods.push(convert_method(x, &proc.1));
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

fn convert_method(property: &impl ParsedMethod, procedure: &original::Procedure) -> output::Method {
    output::Method {
        id: procedure.id,
        procedure: property.original_procedure_name(),
        name: property.function_name(),
        decoder_function: decoder_function(&procedure),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure),
    }
}

fn decoder_function(procedure: &original::Procedure) -> String {
    match &procedure.return_type {
        Some(return_type) => {
            match &return_type.code {
                original::Code::String => "decode_string".to_string(),
                original::Code::Bool => "decode_bool".to_string(),
                original::Code::Float => "decode_float".to_string(),
                original::Code::Double => "decode_double".to_string(),
                original::Code::Sint32 => "decode_sint32".to_string(),
                original::Code::Uint32 => "decode_uint32".to_string(),
                original::Code::Enumeration => "decode_enumeration".to_string(),
                original::Code::List => "decode_list".to_string(),
                original::Code::Dictionary => "decode_dictionary".to_string(),
                original::Code::Set => "decode_set".to_string(),
                original::Code::Tuple => "decode_tuple".to_string(),
                original::Code::Class => "decode_class".to_string(),
            }
        },
        None => "decode_none".to_string()
    }
}

fn return_type_signature(procedure: &original::Procedure) -> String {
    match &procedure.return_type {
        Some(return_type) => {
            match &return_type.code {
                original::Code::String => "String".to_string(),
                original::Code::Bool => "bool".to_string(),
                original::Code::Float => "f32".to_string(),
                original::Code::Double => "f64".to_string(),
                original::Code::Sint32 => "i32".to_string(),
                original::Code::Uint32 => "u32".to_string(),
                original::Code::Enumeration => "(/*enum*/)".to_string(),
                original::Code::List => "(/*list*/)".to_string(),
                original::Code::Dictionary => "(/*dict*/)".to_string(),
                original::Code::Set => "(/*set*/)".to_string(),
                original::Code::Tuple => "(/*tuple*/)".to_string(),
                original::Code::Class => {
                    return_type.name.clone().unwrap() + "<'a>"
                },
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
            match &return_type.code {
                original::Code::String |
                original::Code::Bool |
                original::Code::Float |
                original::Code::Double | 
                original::Code::Sint32 |
                original::Code::Uint32 | 
                original::Code::Enumeration | 
                original::Code::List |
                original::Code::Dictionary |
                original::Code::Set |
                original::Code::Tuple => "return_value".to_string(),
                original::Code::Class => {
                    format!("{}{{id: return_value, conn: &self.conn}}", return_type.name.clone().unwrap())
                },
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
            procedure: "WarpTo".to_string(),
            name: "WarpTo".to_string(),
        });
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_service_getter() {
        let result = get_procedure_type("get_WarpMode");
        let expected = ProcedureType::PropertyGetter(ServiceProperty {
            procedure: "get_WarpMode".to_string(),
            name: "WarpMode".to_string(),
            prefix: "get".to_string(),
        });
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_class_method() {
        let result = get_procedure_type("AutoPilot_Engage");
        let expected = ProcedureType::ClassMethod(ClassMethod {
            procedure: "AutoPilot_Engage".to_string(),
            class: "AutoPilot".to_string(),
            method: "Engage".to_string(),
        });
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_class_property_getter() {
        let result = get_procedure_type("Vessel_get_Type");
        let expected = ProcedureType::ClassPropertyGetter(ClassProperty {
            procedure: "Vessel_get_Type".to_string(),
            class: "Vessel".to_string(),
            property: "Type".to_string(),
            prefix: "get".to_string(),
        });
        assert_eq!(result, expected);
    }

}