use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use std::iter::Iterator;

use serde::de::{self, Deserializer, Visitor};
use serde_derive::Deserialize;

/// Parameter type internal reprensentation
///
/// Example:
/// ```
/// let from : String = "u8".into();
/// if let Some(t) = InstFredbackParameterType::try_from(from) {
///    
/// } else { 
///    panic!("Failed decoding");
/// }
/// ```
#[derive(PartialEq,Eq,Clone,Copy)]
pub enum InstFeedbackParameterType 
{
    Uint64,
    Uint32,
    Uint16,
    Uint8,
    Int64,
    Int32,
    Int16,
    Int8,
    Bool,
    String
}

impl InstFeedbackParameterType {
    pub fn to_rust_type_string(&self) -> String {
        match self {
            Self::String => "String",
            Self::Uint64 => "u64",
            Self::Uint32 => "u32",
            Self::Uint16 => "u16",
            Self::Uint8 => "u8",
            Self::Int64 => "i64",
            Self::Int32 => "i32",
            Self::Int16 => "i16",
            Self::Int8 => "i8",
            Self::Bool => "bool"
        }.into()
    }

    pub fn to_cpp_type_string(&self) -> String {
        match self {
            Self::String => "char *",
            Self::Uint64 => "uint64_t",
            Self::Uint32 => "uint32_t",
            Self::Uint16 => "uint16_t",
            Self::Uint8 => "uint8_t",
            Self::Int64 => "int64_t",
            Self::Int32 => "int32_t",
            Self::Int16 => "int16_t",
            Self::Int8 => "int8_t",
            Self::Bool => "bool"
        }.into()
    }

    pub fn size(&self) -> usize {
        match self {
            Self::String => 0,
            Self::Uint64 => 8,
            Self::Uint32 => 4,
            Self::Uint16 => 2,
            Self::Uint8 => 1,
            Self::Int64 => 8,
            Self::Int32 => 4,
            Self::Int16 => 2,
            Self::Int8 => 1,
            Self::Bool => 1
        }
    }

    fn toString(&self) -> String{
        match self {
            Self::String => "String".into(),
            Self::Uint64 => "Uint64".into(),
            Self::Uint32 => "Uint32".into(),
            Self::Uint16 => "Uint16".into(),
            Self::Uint8  => "Uint8".into(),
            Self::Int64  => "Int64".into(),
            Self::Int32  => "Int32".into(),
            Self::Int16  => "Int16".into(),
            Self::Int8   => "Int8".into(),
            Self::Bool   => "Bool".into()
        }
    }
}

struct InstFeedbackParameterTypeVisitor;
impl<'de> Visitor<'de> for InstFeedbackParameterTypeVisitor {
    type Value = InstFeedbackParameterType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing type [uXX, iXX or string]")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match InstFeedbackParameterType::try_from(String::from(value))
        {
            Ok(v) => Ok(v),
            Err(e) => Err(E::custom(e))
        }
    }
}

impl<'de> de::Deserialize<'de> for InstFeedbackParameterType {
    fn deserialize<D>(deserializer: D) -> Result<InstFeedbackParameterType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(InstFeedbackParameterTypeVisitor)
    }
}

impl TryFrom<String> for InstFeedbackParameterType {
    type Error = String;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        match from.to_lowercase().as_str() {
            "u8" | "uint8" | "byte" => Ok(Self::Uint8),
            "u16" | "uint16" => Ok(Self::Uint16),
            "u32" | "uint32" => Ok(Self::Uint32),
            "u64" | "uint64" => Ok(Self::Uint64),
            "i8" | "int8" => Ok(Self::Int8),
            "i16" | "int16" => Ok(Self::Int16),
            "i32" | "int32" => Ok(Self::Int32),
            "i64" | "int64" => Ok(Self::Int64),
            "string" | "str" => Ok(Self::String),
            "bool" | "boolean" => Ok(Self::Bool),
            _ => Err(format!("Unknown type {}", from))
        }
    }
}

#[derive(Deserialize)]
pub struct InstFeedbackParameter
{
    pub name: String,
    pub description: String,
    pub data_type: InstFeedbackParameterType
}

#[derive(Deserialize)]
pub struct InstFeedback {
    pub description: String,
    pub parameters: Vec<InstFeedbackParameter>
}

#[derive(Deserialize)]
pub struct Codes {
    pub name: String,
    pub instruction: Option<InstFeedback>,
    pub feedback: Option<InstFeedback>,
}

#[derive(Deserialize)]
pub struct CodesFile {
    pub codes: BTreeMap<u32, Codes>
}


#[cfg(test)]
mod test {
    use crate::yaml_parser::*;

    #[test]
    fn parsing_fails() -> Result<(), String> {
        let from : String = "toto".into();
        if let Ok(_result) = InstFeedbackParameterType::try_from(from)
        {
            Err("Should panic with \"toto\"".into())
        }
        else {
            Ok(())
        }
    }

    fn construct_from_string_and_match_type(stringType: &str, expect: InstFeedbackParameterType) -> Result<(), String> {
        let parsed_type = construct_parameter_from_char_array(stringType);
        types_match_or_error(expect, parsed_type)
    }

    fn construct_parameter_from_char_array(name: &str) -> Result<InstFeedbackParameterType, String> {
        let from: String = name.into();
        InstFeedbackParameterType::try_from(from)
    }

    fn types_match_or_error(expect: InstFeedbackParameterType, actual: Result<InstFeedbackParameterType, String>) -> Result<(), String> {
        match actual {
            Ok(actual_type) if (actual_type == expect) => Ok(()),
            Ok(other_type) => Err(format!("Wrong type: {:?} instead of {:?}", other_type.toString(), expect.toString())),
            Err(message) => Err(message)
        } 
    }

    #[test]
    fn type_uint8_doesnt_give_uint16() -> Result<(), String> {
        if let Ok(()) = construct_from_string_and_match_type("uint8", InstFeedbackParameterType::Uint16) {
            Err("Matching function must be wrong".into())
        } else {
            Ok(())
        }
    }

    fn construct_and_match_type_from_array(input_iterator: Box<dyn Iterator<Item=&str>>, expect: InstFeedbackParameterType) ->
    Result<(), String> {
        let test_results = input_iterator.map(|type_string| construct_from_string_and_match_type(type_string, expect));
        test_results.reduce(|previous, current| previous.and(current) ).unwrap()
    }


    #[test]
    fn type_uint8_gives_uint8() -> Result<(), String> {
        let input_types_iter = ["uint8", "Uint8", "U8", "u8", "byte"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint8)
    }

    #[test]
    fn type_uint16_gives_uint16() -> Result<(), String> {
        let input_types_iter = ["uint16", "Uint16", "U16", "u16"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint16)
    }

    #[test]
    fn type_uint32_gives_uint32() -> Result<(), String> {
        let input_types_iter = ["uint32", "Uint32", "U32", "u32"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint32)
    }

    #[test]
    fn type_uint64_gives_uint64() -> Result<(), String> {
        let input_types_iter = ["uint64", "Uint64", "U64", "u64"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint64)
    }

    #[test]
    fn type_int8_gives_int8() -> Result<(), String> {
        let input_types_iter = ["int8", "Int8", "I8", "i8"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int8)
    }

    #[test]
    fn type_int16_gives_int16() -> Result<(), String> {
        let input_types_iter = ["int16", "Int16", "I16", "i16"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int16)
    }

    #[test]
    fn type_int32_gives_int32() -> Result<(), String> {
        let input_types_iter = ["int32", "Int32", "I32", "i32"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int32)
    }

    #[test]
    fn type_int64_gives_int64() -> Result<(), String> {
        let input_types_iter = ["int64", "Int64", "I64", "i64"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int64)
    }
}
