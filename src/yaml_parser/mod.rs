use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, Deserializer, Visitor};
use serde_derive::Deserialize;

mod types;

pub use types::InstFeedbackParameterType;

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
mod test;
