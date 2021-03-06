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
    let mut service_getters_setters = Vec::<output::Method>::new();
    let mut classes = HashMap::<String, output::Class>::new();
    let mut enumerations = Vec::<output::Enumeration>::new();
    
    // create maps for all classes
    for class in &input_structure.classes {
        classes.insert(class.0.clone(), output::Class {
            name: class.0.clone(),
            methods: vec![],
            getters_setters: vec![],
            static_methods: vec![],
        });
    }    

    // parse procedures
    for proc in &input_structure.procedures {
        let procedure_type = get_procedure_type(proc.0);
        match &procedure_type {
            ProcedureType::Standard(x) => {
                service_methods.push(convert_method(x, &proc.1, false));
            },
            ProcedureType::PropertyGetter(x) => {
                service_getters_setters.push(convert_method(x, &proc.1, false));
            },
            ProcedureType::PropertySetter(x) => {
                service_getters_setters.push(convert_method(x, &proc.1, false));
            },
            ProcedureType::ClassMethod(x) => {
                // add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().methods.push(convert_method(x, &proc.1, false));
            },
            ProcedureType::ClassPropertyGetter(x) => {
                // add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().getters_setters.push(convert_method(x, &proc.1, false));
            },
            ProcedureType::ClassPropertySetter(x) => {
                // add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().getters_setters.push(convert_method(x, &proc.1, false));
            },
            ProcedureType::StaticClassMethod(x) => {
                // add_class_if_nonexistent(&mut classes, &x.class);
                classes.get_mut(&x.class).unwrap().static_methods.push(convert_method(x, &proc.1, true));
            },
            ProcedureType::Unknown => {}
        }
    }
    
    // parse enums
    for e in &input_structure.enumerations {
        let enum_values: Vec<output::EnumerationValue> = e.1.values.iter()
            .map(|v| output::EnumerationValue {
                id: v.value,
                name: v.name.clone(),
            })
            .collect();
        let enumeration = output::Enumeration {
            name: e.0.to_string(),
            values: enum_values,
        };
        enumerations.push(enumeration);
    }
    
    // Sort lists
    service_methods.sort();
    service_getters_setters.sort();
    
    for (_, class) in &mut classes {
        class.methods.sort();
        class.getters_setters.sort();
        class.static_methods.sort();
    }
    
    output::OutputStructure {
        methods: service_methods,
        getters_setters: service_getters_setters,
        classes: classes,
        enumerations: enumerations,
    }
}

fn convert_method(property: &impl ParsedMethod, procedure: &original::Procedure, is_static: bool) -> output::Method {
    output::Method {
        id: procedure.id,
        procedure: property.original_procedure_name(),
        name: property.function_name(),
        arguments_signature: arguments_signature(&procedure, is_static),
        arguments: convert_arguments(&procedure),
        decoder_function: decoder_function(&procedure),
        return_type_signature: return_type_signature(&procedure),
        return_value: return_value(&procedure, is_static),
    }
}

fn arguments_signature(procedure: &original::Procedure, is_static: bool) -> String {
    let first_argument = if is_static { "conn: &'a Connection" } else { "&'a self" }.to_string();
    let arguments: Vec<String> = procedure.parameters.iter()
        .filter(|param| param.name != "this")
        .map(|param|
            param.name.to_case(Case::Snake) + ": " +
            argument_type(param).as_str()
        )
        .collect();

    let arguments = [Vec::from([first_argument]), arguments].concat();
    arguments.join(", ")
}

fn argument_type(parameter: &original::Parameter) -> String {
    match parameter.r#type.code {
        original::Code::String => "String".to_string(),
        original::Code::Bool => "bool".to_string(),
        original::Code::Float => "f32".to_string(),
        original::Code::Double => "f64".to_string(),
        original::Code::Sint32 => "i32".to_string(),
        original::Code::Uint32 => "u32".to_string(),
        original::Code::Enumeration => parameter.r#type.name.clone().unwrap(),
        original::Code::List => "(/*list*/)".to_string(),
        original::Code::Dictionary => "(/*dict*/)".to_string(),
        original::Code::Set => "(/*set*/)".to_string(),
        original::Code::Tuple => "(/*tuple*/)".to_string(),
        original::Code::Class => {
            "&".to_string() + parameter.r#type.name.clone().unwrap().as_str() + "<'_>"
        },
    }
}

fn convert_arguments(procedure: &original::Procedure) -> Vec<output::Argument> {
    let mut arguments = Vec::new();
    let mut position = 0;
    for p in &procedure.parameters {
        arguments.push(convert_single_argument(&p, position));
        position += 1;
    }
    arguments
}

fn convert_single_argument(parameter: &original::Parameter, position: u64) -> output::Argument {
    if parameter.name == "this" {
        return output::Argument {
            position: 0,
            encoder_function: "encode_u64".to_string(),
            value: "self.id".to_string(),
        };
    }
    let encoder_function = match parameter.r#type.code {
        original::Code::String => "encode_string".to_string(),
        original::Code::Bool => "encode_bool".to_string(),
        original::Code::Float => "encode_float".to_string(),
        original::Code::Double => "encode_double".to_string(),
        original::Code::Sint32 => "encode_sint32".to_string(),
        original::Code::Uint32 => "encode_uint32".to_string(),
        original::Code::Enumeration => "encode_u64".to_string(),
        original::Code::List => "encode_list".to_string(),
        original::Code::Dictionary => "encode_dictionary".to_string(),
        original::Code::Set => "encode_set".to_string(),
        original::Code::Tuple => "encode_tuple".to_string(),
        original::Code::Class => "encode_u64".to_string(),
    };
    let value = match parameter.r#type.code {
        original::Code::Class => parameter.name.to_case(Case::Snake) + ".id",
        original::Code::Enumeration => parameter.name.to_case(Case::Snake) + " as u64",
        _ => parameter.name.to_case(Case::Snake),
    };
    output::Argument {
        position,
        encoder_function,
        value,
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
                original::Code::List => {
                    let types = (&return_type.types).clone().unwrap();
                    let list_type_string = get_list_type(&types);
                    format!("decode_list::<{}>", list_type_string).to_string()
                },
                original::Code::Dictionary => "decode_dictionary".to_string(),
                original::Code::Set => "decode_set".to_string(),
                original::Code::Tuple => "decode_tuple".to_string(),
                original::Code::Class => "decode_class".to_string(),
            }
        },
        None => "decode_none".to_string()
    }
}

fn get_list_type(types: &Vec<original::Type>) -> String {
    let list_type = types.get(0).unwrap();
    match list_type.code {
        original::Code::String => "String".to_string(),
        original::Code::Class => {
            list_type.name.clone().unwrap() + "<'_>"
        },
        _ => "(/*list*/)".to_string()
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
                original::Code::List => {
                    let types = (&return_type.types).clone().unwrap();
                    let list_type_string = get_list_type(&types);
                    format!("Vec<{}>", list_type_string).to_string()
                },
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

fn return_value(procedure: &original::Procedure, is_static: bool) -> String {
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
                    format!("{}{{id: return_value, conn: {}}}",
                        return_type.name.clone().unwrap(),
                        if is_static { "&conn" } else { "&self.conn" })
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