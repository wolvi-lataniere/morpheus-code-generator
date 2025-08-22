use crate::file_generator::{FileGenerator, language_models};
use crate::yaml_parser::{self};
use std::fs::File;
use std::io::{self, Write};

const RUST_TEMPLATE: &str = include_str!("./templates/rust_template.rs");

pub struct RustFileGenerator {
    writer: Box<dyn Write>,
}

impl FileGenerator for RustFileGenerator {
    fn build_file(&mut self, codes: &yaml_parser::CodesFile) -> Result<(), io::Error> {
        // Show some Rust code
        self.writer.write_all(
            self.process_template(RUST_TEMPLATE, &language_models::RustLanguageModel {}, codes)
                .as_bytes(),
        )
    }
}

impl RustFileGenerator {
    pub fn new(file_name: String) -> Result<Self, io::Error> {
        let file = Box::new(File::create(file_name)?);
        Ok(Self { writer: file })
    }
}

#[cfg(test)]
include!("templates/rust_template.rs");
#[cfg(test)]
include!("templates/rust_template_test.rs");
