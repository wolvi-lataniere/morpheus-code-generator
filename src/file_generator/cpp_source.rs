use crate::yaml_parser::{self};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::file_generator::{FileGenerator, language_models};

const C_SOURCE_TEMPLATE: &str = include_str!("./templates/c_template.c");

pub struct CppFileGenerator {
    writer: Box<dyn Write>,
    headerfile_name: String,
}

impl FileGenerator for CppFileGenerator {
    fn build_file(&mut self, codes: &yaml_parser::CodesFile) -> Result<(), io::Error> {
        self.writer.write_all(
            self.process_template(
                C_SOURCE_TEMPLATE,
                &language_models::CppLanguageModel {
                    headerfile_name: Some(self.headerfile_name.clone()),
                },
                codes,
            )
            .as_bytes(),
        )
    }
}

impl CppFileGenerator {
    pub fn new(
        source_file_name: &String,
        header_file: &Option<String>,
    ) -> Result<CppFileGenerator, io::Error> {
        //// Create source code file
        let file = File::create(source_file_name)?;

        let headerfile_name = Path::new(&header_file.clone().unwrap_or(source_file_name.clone()))
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".cpp", ".h")
            .replace(".c", ".h");

        Ok(CppFileGenerator {
            writer: Box::new(file),
            headerfile_name,
        })
    }
}
