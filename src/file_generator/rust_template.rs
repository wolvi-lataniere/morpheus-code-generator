use crate::file_generator::FileGenerator;
use crate::yaml_parser::{self, ParameterType};
use std::fs::File;
use std::io::{self, Write};

const FILE_HEADER: &[u8] = include_bytes!("./templates/rust_template.rs");

impl ParameterType {
    fn to_typesenum_name(&self) -> &str {
        match self {
            ParameterType::Int8 => "I8",
            ParameterType::Uint8 => "U8",
            ParameterType::Int16 => "I16",
            ParameterType::Uint16 => "U16",
            ParameterType::Int32 => "I32",
            ParameterType::Uint32 => "U32",
            ParameterType::Int64 => "I64",
            ParameterType::Uint64 => "U64",
            ParameterType::Bool => "Bool",
            ParameterType::String => "Str",
        }
    }

    fn to_typesenum_parsing_funtion_type(&self) -> &str {
        match self {
            ParameterType::Int8 => "i8",
            ParameterType::Uint8 => "u8",
            ParameterType::Int16 => "i16",
            ParameterType::Uint16 => "u16",
            ParameterType::Int32 => "i32",
            ParameterType::Uint32 => "u32",
            ParameterType::Int64 => "i64",
            ParameterType::Uint64 => "u64",
            ParameterType::Bool => "bool",
            ParameterType::String => "string",
        }
    }
}

pub struct RustFileGenerator {
    writer: Box<dyn Write>,
}

impl FileGenerator for RustFileGenerator {
    fn build_file(&mut self, codes: &yaml_parser::CodesFile) -> Result<(), io::Error> {
        // Show some Rust code
        self.writer.write_all(FILE_HEADER)?;

        self.declare_instructions(codes)?;

        self.declare_feedbacks(codes)?;

        self.implement_feedbacks(codes)?;

        self.implement_instructions(codes)?;

        Ok(())
    }
}

impl RustFileGenerator {
    fn map_instfeedback_list_and_type(inst: &yaml_parser::InstFeedback) -> String {
        format!(
            "{{{}}}",
            inst.parameters
                .iter()
                .map(|v| format!("{}: {}", v.name, v.data_type.to_rust_type_string()))
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    pub fn new(file_name: String) -> Result<Self, io::Error> {
        let file = Box::new(File::create(file_name)?);
        Ok(Self { writer: file })
    }

    fn declare_instructions(&mut self, codes: &crate::CodesFile) -> Result<(), io::Error> {
        self.writer.write_all(
            r#"
#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
pub enum Instructions {
    "#
            .as_bytes(),
        )?;
        self.writer.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(_k, code)| {
                    code.instruction.clone().map(|inst| {
                        format!(
                            "{}{},     // {}",
                            code.name,
                            Self::map_instfeedback_list_and_type(&inst),
                            inst.description
                        )
                    })
                })
                .collect::<Vec<String>>()
                .join("\n\t")
                .as_bytes(),
        )?;

        self.writer.write_all(
            r#"
}


"#
            .as_bytes(),
        )?;

        Ok(())
    }

    fn declare_feedbacks(&mut self, codes: &crate::CodesFile) -> Result<(), io::Error> {
        self.writer.write_all(
            r#"
#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
pub enum Feedbacks {
    "#
            .as_bytes(),
        )?;
        self.writer.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(_k, code)| {
                    code.feedback.clone().map(|inst| {
                        format!(
                            "{}{}",
                            code.name,
                            Self::map_instfeedback_list_and_type(&inst)
                        )
                    })
                })
                .collect::<Vec<String>>()
                .join(",\n\t")
                .as_bytes(),
        )?;
        self.writer.write_all(
            r#"
}



"#
            .as_bytes(),
        )?;
        Ok(())
    }

    fn implement_feedbacks(&mut self, codes: &crate::CodesFile) -> Result<(), io::Error> {
        self.writer.write_all(
            r#"
impl Feedbacks {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            "#
            .as_bytes(),
        )?;
        self.writer.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.feedback
                        .clone()
                        .map(|code_fb| build_frame_from_fields(id, code, &code_fb))
                })
                .collect::<Vec<Vec<u8>>>()
                .join(",\n\t\t\t".as_bytes())
                .as_slice(),
        )?;
        self.writer.write_all(
            r#"
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, TypesEnumError> {
        match bytes[0] {
            "#
            .as_bytes(),
        )?;
        self.writer.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.feedback
                        .clone()
                        .map(|code_fb| parse_frame_to_fields(id, code, &code_fb))
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t")
                .as_bytes(),
        )?;
        self.writer.write_all(
            r#",
    _ => Err(TypesEnumError::UnknownCode)
        }
    }
}
"#
            .as_bytes(),
        )?;
        Ok(())
    }

    fn implement_instructions(&mut self, codes: &crate::CodesFile) -> Result<(), io::Error> {
        self.writer.write_all(
            r#"

impl Instructions {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
    "#
            .as_bytes(),
        )?;
        self.writer.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.instruction
                        .clone()
                        .map(|code_fb| build_frame_from_fields(id, code, &code_fb))
                })
                .collect::<Vec<Vec<u8>>>()
                .join(",\n\t\t\t".as_bytes())
                .as_slice(),
        )?;
        self.writer.write_all(
            r#"
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, TypesEnumError> {
        match bytes[0] {
            "#
            .as_bytes(),
        )?;
        self.writer.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.instruction
                        .clone()
                        .map(|code_fb| parse_frame_to_fields(id, code, &code_fb))
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t")
                .as_bytes(),
        )?;
        self.writer.write_all(
            r#",
    _ => Err(TypesEnumError::UnknownCode)
        }
    }
}
"#
            .as_bytes(),
        )?;
        Ok(())
    }
}

fn parse_frame_to_fields(id: u32, code: &crate::Codes, code_fb: &crate::InstFeedback) -> String {
    let params_parsing = &code_fb
        .parameters
        .iter()
        .map(|param| {
            format!(
                r#"let ({}, bytes) = TypesEnum::{}_from_buffer(bytes)?;
"#,
                param.name,
                param.data_type.to_typesenum_parsing_funtion_type(),
            )
            .as_bytes()
            .to_vec()
        })
        .collect::<Vec<Vec<u8>>>()
        .join("\t\t\t\t".as_bytes());

    format!(
        r#"{}u8 => {{
                let bytes = &bytes[1..];
                {}
                Ok(Self::{}{{{}}})
            }}
                "#,
        id,
        String::from_utf8(params_parsing.clone()).unwrap(),
        &code.name,
        code_fb
            .parameters
            .iter()
            .map(|name| name.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn build_frame_from_fields(id: u32, code: &crate::Codes, code_fb: &crate::InstFeedback) -> Vec<u8> {
    let params = &code_fb
        .parameters
        .iter()
        .map(|param| {
            format!(
                "TypesEnum::{}({}).write_to_buffer().as_slice()",
                param.data_type.to_typesenum_name(),
                param.name
            )
            .as_bytes()
            .to_vec()
        })
        .collect::<Vec<Vec<u8>>>()
        .join(",".as_bytes());

    [
        format!(
            "Self::{}{{{}}} => vec![&[{}u8] as &[u8],",
            &code.name,
            code_fb
                .parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
                .join(","),
            id
        )
        .as_bytes(),
        params.as_slice(),
        "].concat()".as_bytes(),
    ]
    .concat()
}

#[cfg(test)]
include!("templates/rust_template.rs");
#[cfg(test)]
include!("templates/rust_template_test.rs");
