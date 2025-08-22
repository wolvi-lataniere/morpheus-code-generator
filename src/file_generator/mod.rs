use std::io;

mod cpp_header;
mod cpp_source;
mod rust_template;

use crate::yaml_parser;
pub use cpp_header::CppHeaderGenerator;
pub use cpp_source::CppFileGenerator;
pub use rust_template::RustFileGenerator;

pub trait FileGenerator {
    fn build_file(&mut self, codes: &yaml_parser::CodesFile) -> Result<(), io::Error>;
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
    fn file_header(&self) -> String;
    fn declare_feedbacks(&self, codes: &yaml_parser::CodesFile) -> String;
    fn declare_instructions(&self, codes: &yaml_parser::CodesFile) -> String;
    fn implement_feedbacks(&self, codes: &yaml_parser::CodesFile) -> String;
    fn implement_instructions(&self, codes: &yaml_parser::CodesFile) -> String;
}
