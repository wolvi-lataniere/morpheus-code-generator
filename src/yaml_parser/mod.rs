use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, Deserializer, Visitor, Unexpected};
use serde_derive::Deserialize;

mod types;

pub use types::ParameterType;

struct ParameterTypeVisitor;
impl<'de> Visitor<'de> for ParameterTypeVisitor {
    type Value = ParameterType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing type [uXX, iXX, string or bool]")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match ParameterType::try_from(String::from(value))
        {
            Ok(v) => Ok(v),
            Err(e) => Err(E::invalid_value(Unexpected::Str(value), &self))
        }
    }
}

impl<'de> de::Deserialize<'de> for ParameterType {
    fn deserialize<D>(deserializer: D) -> Result<ParameterType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ParameterTypeVisitor)
    }
}

#[derive(Deserialize,Debug)]
pub struct InstFeedbackParameter
{
    pub name: String,
    pub description: String,
    pub data_type: ParameterType
}

#[derive(Deserialize,Debug)]
pub struct InstFeedback {
    pub description: String,
    pub parameters: Vec<InstFeedbackParameter>
}

#[derive(Deserialize,Debug)]
pub struct Codes {
    pub name: String,
    pub instruction: Option<InstFeedback>,
    pub feedback: Option<InstFeedback>,
}

#[derive(Deserialize,Debug)]
pub struct CodesFile {
    pub codes: BTreeMap<u32, Codes>
}


#[cfg(test)]
mod test;
