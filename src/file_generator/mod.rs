use std::io;

mod cpp_header;
mod cpp_source;
mod rust_template;

use crate::yaml_parser;
pub use cpp_header::CppHeaderGenerator;
pub use cpp_source::CppFileGenerator;
pub use rust_template::build_rust_source;

pub trait FileGenerator {
    fn write_header(&mut self) -> Result<(), io::Error>;
    fn write_enumerations(
        &mut self,
        parameters: Vec<(&u32, &yaml_parser::Codes)>,
    ) -> Result<(), io::Error>;
    fn write_footer(&mut self) -> Result<(), io::Error>;
}
