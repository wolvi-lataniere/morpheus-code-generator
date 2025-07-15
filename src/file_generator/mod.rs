use std::io;

mod cpp_template;
mod rust_template;

use crate::yaml_parser;
pub use cpp_template::{CppHeaderGenerator, build_cpp_source};
pub use rust_template::build_rust_source;

pub trait FileGenerator {
    fn write_header(&mut self) -> Result<(), io::Error>;
    fn write_enumerations(
        &mut self,
        parameters: Vec<(&u32, &yaml_parser::Codes)>,
    ) -> Result<(), io::Error>;
    fn write_footer(&mut self) -> Result<(), io::Error>;
}
