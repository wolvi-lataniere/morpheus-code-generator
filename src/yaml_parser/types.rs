use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

/// Parameter type internal reprensentation
///
/// Example:
/// ```
/// let from : String = "u8".into();
/// if let Some(t) = ParameterType::try_from(from) {
///    
/// } else {
///    panic!("Failed decoding");
/// }
/// ```
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ParameterType {
    Uint64,
    Uint32,
    Uint16,
    Uint8,
    Int64,
    Int32,
    Int16,
    Int8,
    Bool,
    String,
}

impl ParameterType {
    pub fn to_rust_type_string(self) -> String {
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
            Self::Bool => "bool",
        }
        .into()
    }

    pub fn to_cpp_type_string(self) -> String {
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
            Self::Bool => "bool",
        }
        .into()
    }

    pub fn size(self) -> usize {
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
            Self::Bool => 1,
        }
    }
}

impl fmt::Display for ParameterType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::String => "String",
            Self::Uint64 => "Uint64",
            Self::Uint32 => "Uint32",
            Self::Uint16 => "Uint16",
            Self::Uint8 => "Uint8",
            Self::Int64 => "Int64",
            Self::Int32 => "Int32",
            Self::Int16 => "Int16",
            Self::Int8 => "Int8",
            Self::Bool => "Bool",
        })
    }
}

impl Debug for ParameterType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        formatter.write_str(self.to_string().as_str())
    }
}

impl TryFrom<String> for ParameterType {
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
            _ => Err(format!("Unknown type {}", from)),
        }
    }
}
