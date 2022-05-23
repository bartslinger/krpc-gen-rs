use std::collections::HashMap;
use regex::Regex;
use convert_case::{Case, Casing};
use crate::original::{self, Type};
use crate::original::ReturnType;
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
        self.name.to_case(Case::Camel)
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
        self.prefix.clone() + self.name.to_case(Case::UpperCamel).as_str()
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
        self.method.to_case(Case::Camel)
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
        self.prefix.clone() + self.property.to_case(Case::UpperCamel).as_str()
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
                    prefix: "get".to_string(),
                });
            },
            "set" => {
                return ProcedureType::ClassPropertySetter(ClassProperty {
                    procedure: procedure_name.to_string(),
                    class: (&cap[1]).to_string(),
                    property: (&cap[3]).to_string(),
                    prefix: "set".to_string(),
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
                    prefix: "get".to_string(),
                })
            },
            "set" => {
                return ProcedureType::PropertySetter(ServiceProperty {
                    procedure: procedure_name.to_string(),
                    name: (&cap[2]).to_string(),
                    prefix: "set".to_string(),
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
        decoder_function: decoder_function(&procedure.return_type, "result.value".to_string()),
        before_return: before_return(&procedure.return_type),
        return_type_signature: return_type_signature(&procedure.return_type),
        return_value: return_value(&procedure, is_static),
    }
}

fn arguments_signature(procedure: &original::Procedure, is_static: bool) -> String {
    let arguments: Vec<String> = procedure.parameters.iter()
        .filter(|param| param.name != "this")
        .map(|param|
            param.name.to_case(Case::Camel) + ": " +
            argument_type(&param.r#type).as_str()
        )
        .collect();

    arguments.join(", ")
}

fn argument_type(argument: &Type) -> String {
    match argument.code {
        original::Code::String => "string".to_string(),
        original::Code::Bool => "boolean".to_string(),
        original::Code::Float => "number".to_string(),
        original::Code::Double => "number".to_string(),
        original::Code::Sint32 => "number".to_string(),
        original::Code::Uint32 => "number".to_string(),
        original::Code::Enumeration => argument.name.clone().unwrap(),
        original::Code::List => "void /*list*/".to_string(),
        original::Code::Dictionary => "void /*dict*/".to_string(),
        original::Code::Set => "void /*set*/".to_string(),
        original::Code::Tuple => {
            let types = argument.types.clone().unwrap();
            let tuple_type: Vec<String> = types.iter()
                .map(|x| argument_type(x))
                .collect();
            format!("[{}]", tuple_type.join(", ")).to_string()
        },
        original::Code::Class => argument.name.clone().unwrap(),
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
            encoder_function: "encoding.encodeVarint64".to_string(),
            value: "this.id".to_string(),
        };
    }
    let encoder_function = match parameter.r#type.code {
        original::Code::String => "encoding.encodeString".to_string(),
        original::Code::Bool => "encoding.encodeBool".to_string(),
        original::Code::Float => "encoding.encodeFloat".to_string(),
        original::Code::Double => "encoding.encodeDouble".to_string(),
        original::Code::Sint32 => "encoding.encodeSint32".to_string(),
        original::Code::Uint32 => "encoding.encodeUint32".to_string(),
        original::Code::Enumeration => "encoding.encodeVarint64".to_string(),
        original::Code::List => "encoding.encodeList".to_string(),
        original::Code::Dictionary => "encoding.encodeDict".to_string(),
        original::Code::Set => "encoding.encodeSet".to_string(),
        original::Code::Tuple => "encoding.encodeTuple".to_string(),
        original::Code::Class => "encoding.encodeVarint64".to_string(),
    };
    let value = match parameter.r#type.code {
        original::Code::Class => parameter.name.to_case(Case::Camel) + ".id",
        original::Code::Enumeration => format!("Long.fromInt({}.valueOf())", parameter.name.to_case(Case::Camel)),
        _ => parameter.name.to_case(Case::Camel),
    };
    output::Argument {
        position,
        encoder_function,
        value,
    }
}

fn decoder_function(return_type: &Option<ReturnType>, input: String) -> String {
    match return_type {
        Some(return_type) => {
            match &return_type.code {
                original::Code::String => format!("encoding.decodeString(this.conn, {})", input).to_string(),
                original::Code::Bool => format!("encoding.decodeBool(this.conn, {})", input).to_string(),
                original::Code::Float => format!("encoding.decodeFloat(this.conn, {})", input).to_string(),
                original::Code::Double => format!("encoding.decodeDouble(this.conn, {})", input).to_string(),
                original::Code::Sint32 => format!("encoding.decodeSint32(this.conn, {})", input).to_string(),
                original::Code::Uint32 => format!("encoding.decodeUint32(this.conn, {})", input).to_string(),
                original::Code::Enumeration => format!("encoding.decodeEnum(this.conn, {})", input).to_string(),
                original::Code::List => {
                    let types = (&return_type.types).clone().unwrap();
                    let list_item_type = types.get(0).unwrap();
                    let list_item_decoder_function = decoder_function(&Some(list_item_type.clone()), "item".to_string());
                    format!("list.map((item) => {{ return {}}})", list_item_decoder_function).to_string()
                },
                original::Code::Set => {
                    "set".to_string()
                },
                original::Code::Dictionary => {
                    let types = (&return_type.types).clone().unwrap();
                    let key_type = types.get(0).unwrap().clone();
                    let value_type = types.get(1).unwrap().clone();
                    format!("dict.reduce((obj: {}, item) => {{ const key = {}; const value = {}; obj[key] = value; return obj;}}, {{}})", return_type_signature(&Some(return_type.clone())), decoder_function(&Some(key_type), "item.key".to_string()), decoder_function(&Some(value_type), "item.value".to_string())).to_string()
                },
                original::Code::Tuple => {
                    let types = (&return_type.types).clone().unwrap();
                    let test: Vec<String> = types.into_iter().enumerate()
                        .map(|x| decoder_function(&Some(x.1), format!("tuple[{}]", x.0).to_string()))
                        .collect();
                    format!("[{}]", test.join(", ")).to_string()
                },
                original::Code::Class => {
                    format!("{}.decode(this.conn, {})", &return_type.name.clone().unwrap(), input).to_string()
                },
            }
        },
        None => "undefined".to_string()
    }
}

fn before_return(return_type: &Option<ReturnType>) -> String {
    match return_type {
        Some(return_type) => {
            match &return_type.code {
                original::Code::List => {
                    format!("const list = encoding.decodeList(this.conn, result.value).items;").to_string()
                }
                original::Code::Set => {
                    let types = (&return_type.types).clone().unwrap();
                    let set_type = types.get(0).unwrap().clone();
                    format!("const set: {} = new Set(); encoding.decodeSet(this.conn, result.value).items.forEach((item) => {{ set.add({});}});", return_type_signature(&Some(return_type.clone())), decoder_function(&Some(set_type), "item".to_string())).to_string()
                },
                original::Code::Tuple => {
                    format!("const tuple = encoding.decodeTuple(this.conn, result.value).items;").to_string()
                },
                original::Code::Dictionary => {
                    format!("const dict = encoding.decodeDict(this.conn, result.value).entries;").to_string()
                },
                _ => "".to_string(),
            }
        },
        None => "".to_string()
    }
}

fn return_type_signature(return_type: &Option<original::ReturnType>) -> String {
    match return_type {
        Some(return_type) => {
            match &return_type.code {
                original::Code::String => "string".to_string(),
                original::Code::Bool => "boolean".to_string(),
                original::Code::Float => "number".to_string(),
                original::Code::Double => "number".to_string(),
                original::Code::Sint32 => "number".to_string(),
                original::Code::Uint32 => "number".to_string(),
                original::Code::Enumeration => "void /*enum*/".to_string(),
                original::Code::List => {
                    let types = (&return_type.types).clone().unwrap();
                    let list_type = types.get(0).unwrap().clone();
                    format!("{}[]", return_type_signature(&Some(list_type))).to_string()
                },
                original::Code::Dictionary => {
                    let types = return_type.types.clone().unwrap();
                    // first one is always a string apparently
                    let key_type = types.get(0).unwrap().clone();
                    let value_type = types.get(1).unwrap().clone();
                    format!("Record<{}, {}>", return_type_signature(&Some(key_type)), return_type_signature(&Some(value_type))).to_string()
                },
                original::Code::Set => {
                    let types = (&return_type.types).clone().unwrap();
                    let list_type = types.get(0).unwrap().clone();
                    format!("Set<{}>", return_type_signature(&Some(list_type))).to_string()
                },
                original::Code::Tuple => {
                    let types = return_type.types.clone().unwrap();
                    let tuple_type: Vec<String> = types.into_iter()
                        .map(|x| return_type_signature(&Some(x)))
                        .collect();
                    format!("[{}]", tuple_type.join(", ")).to_string()
                },
                original::Code::Class => {
                    return_type.name.clone().unwrap()
                },
            }
        },
        None => {
            "void".to_string()
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