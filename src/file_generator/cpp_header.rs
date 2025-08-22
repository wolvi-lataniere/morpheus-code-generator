use crate::{language_models, yaml_parser};
use std::fs::File;
use std::io::{self, Write};

use crate::file_generator::FileGenerator;

const HEADER_TEMPLATE: &str = include_str!("./templates/c_header.h");

pub struct CppHeaderGenerator {
    file: Box<dyn Write>,
}
impl FileGenerator for CppHeaderGenerator {
    fn build_file(&mut self, codes: &yaml_parser::CodesFile) -> Result<(), io::Error> {
        self.file.write_all(
            self.process_template(
                HEADER_TEMPLATE,
                Box::new(&language_models::CppLanguageModel {
                    headerfile_name: None,
                }),
                codes,
            )
            .as_bytes(),
        )?;

        self.file.flush()?;

        Ok(())
    }
}

impl CppHeaderGenerator {
    pub fn new(header_file_name: &String) -> Result<Self, io::Error> {
        Ok(CppHeaderGenerator {
            file: Box::new(File::create(header_file_name)?),
        })
    }
}
