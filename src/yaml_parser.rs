use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;

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
}