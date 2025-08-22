use std::io;

mod cpp_header;
mod cpp_source;
pub mod language_models;
mod rust_template;

use crate::yaml_parser;
pub use cpp_header::CppHeaderGenerator;
pub use cpp_source::CppFileGenerator;
pub use rust_template::RustFileGenerator;

pub trait FileGenerator {
    fn build_file(&mut self, codes: &yaml_parser::CodesFile) -> Result<(), io::Error>;

    fn process_template(
        &self,
        template: &str,
        model: &dyn LanguageModel,
        codes: &yaml_parser::CodesFile,
    ) -> String {
        template
            .replace(
                "##FEEDBACKS_DECLARATIONS##",
                format!("Feedbacks declarations\n{}", model.declare_feedbacks(codes)).as_str(),
            )
            .replace(
                "##INSTRUCTIONS_DECLARATIONS##",
                format!(
                    "Instructions declaraions\n{}",
                    model.declare_instructions(codes)
                )
                .as_str(),
            )
            .replace(
                "##INSTRUCTIONS_IMPLEMENTATION##",
                format!(
                    "Instructions implementation\n{}",
                    model.implement_instructions(codes)
                )
                .as_str(),
            )
            .replace(
                "##FEEDBACKS_IMPLEMENTATION##",
                format!(
                    "Feedbacks Implementation\n{}",
                    model.implement_feedbacks(codes)
                )
                .as_str(),
            )
            .replace(
                "##CUSTOM_INCLUDES##",
                format!("Custom includes\n{}", model.custom_includes()).as_str(),
            )
            .replace(
                "##CUSTOM_FOOTER##",
                format!("Custom Footer\n{}", model.custom_footer()).as_str(),
            )
    }
}

#[derive(Copy, Clone)]
enum FrameType {
    Instruction,
    Feedback,
}

impl FrameType {
    pub fn short(&self) -> &'static str {
        match self {
            Self::Instruction => "inst",
            Self::Feedback => "fb",
        }
    }

    pub fn long(&self) -> &'static str {
        match self {
            Self::Instruction => "instruction",
            Self::Feedback => "feedback",
        }
    }

    pub fn struct_name(&self) -> &'static str {
        match self {
            Self::Instruction => "Instructions",
            Self::Feedback => "Feedbacks",
        }
    }
}

pub trait LanguageModel {
    fn custom_includes(&self) -> String;
    fn custom_footer(&self) -> String;
    fn declare_feedbacks(&self, codes: &yaml_parser::CodesFile) -> String;
    fn declare_instructions(&self, codes: &yaml_parser::CodesFile) -> String;
    fn implement_feedbacks(&self, codes: &yaml_parser::CodesFile) -> String;
    fn implement_instructions(&self, codes: &yaml_parser::CodesFile) -> String;
}
