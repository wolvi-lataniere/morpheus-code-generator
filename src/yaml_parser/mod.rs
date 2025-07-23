use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, Deserializer, Unexpected, Visitor};
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
        match ParameterType::try_from(String::from(value)) {
            Ok(v) => Ok(v),
            Err(_e) => Err(E::invalid_value(Unexpected::Str(value), &self)),
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InstFeedbackParameter {
    pub name: String,
    pub description: String,
    pub data_type: ParameterType,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InstFeedback {
    pub description: String,
    pub parameters: Vec<InstFeedbackParameter>,
}

#[derive(Deserialize, Debug)]
pub struct Codes {
    pub name: String,
    pub instruction: Option<InstFeedback>,
    pub feedback: Option<InstFeedback>,
}

#[derive(Deserialize, Debug)]
pub struct CodesFile {
    pub codes: BTreeMap<u32, Codes>,
}

impl InstFeedbackParameter {
    pub fn c_parameter_definition(&self) -> String {
        format!("{} {}", self.data_type.to_cpp_type_string(), self.name)
    }

    pub fn c_parameter_definition_with_comment(&self) -> String {
        format!(
            "{} {};\t// {}",
            self.data_type.to_cpp_type_string(),
            self.name,
            self.description
        )
    }
}

impl Codes {
    pub fn get_instructions(&self) -> Option<(String, InstFeedback)> {
        self.instruction
            .as_ref()
            .map(|inst| (self.name.clone(), inst.clone()))
    }

    pub fn get_feedbacks(&self) -> Option<(String, InstFeedback)> {
        self.feedback
            .as_ref()
            .map(|fb| (self.name.clone(), fb.clone()))
    }
}

impl CodesFile {
    pub fn get_instructions(&self) -> Vec<(u32, String, InstFeedback)> {
        self.codes
            .iter()
            .filter_map(|(&code, instruction_code)| {
                instruction_code
                    .get_instructions()
                    .map(|instructions| (code, instructions.0, instructions.1))
            })
            .collect()
    }

    pub fn get_feedbacks(&self) -> Vec<(u32, String, InstFeedback)> {
        self.codes
            .iter()
            .filter_map(|(&code, fb_code)| {
                fb_code
                    .get_feedbacks()
                    .map(|instructions| (code, instructions.0, instructions.1))
            })
            .collect()
    }
}

#[cfg(test)]
mod test;
