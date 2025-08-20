use crate::file_generator::{FileGenerator, FrameType};
use crate::yaml_parser::{self, ParameterType};
use std::fs::File;
use std::io::{self, Write};

include!("templates/rust_template.rs");
include!("templates/rust_template_test.rs");

const FILE_HEADER: &[u8] = include_bytes!("./templates/rust_template.rs");

pub struct RustFileGenerator {
    file: Box<dyn Write>,
}

impl FileGenerator for RustFileGenerator {
    fn build_file(&mut self, codes: &yaml_parser::CodesFile) -> Result<(), io::Error> {
        // Show some Rust code
        self.file.write_all(FILE_HEADER)?;

        // First, create the enumerations declaration.
        self.file.write_all(
            r#"
#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
pub enum Instructions {
    "#
            .as_bytes(),
        )?;

        self.file.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(_k, code)| {
                    code.instruction
                        .clone()
                        .map(|inst| format!("{}{}", code.name, Self::map_inst_feed_rust(&inst)))
                })
                .collect::<Vec<String>>()
                .join(",\n\t")
                .as_bytes(),
        )?;

        self.file.write_all(
            r#"
}


#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
pub enum Feedbacks {
    "#
            .as_bytes(),
        )?;

        self.file.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(_k, code)| {
                    code.feedback
                        .clone()
                        .map(|inst| format!("{}{}", code.name, Self::map_inst_feed_rust(&inst)))
                })
                .collect::<Vec<String>>()
                .join(",\n\t")
                .as_bytes(),
        )?;

        self.file.write_all(
            r#"
}



"#
            .as_bytes(),
        )?;

        // Add to frame implementation
        self.file.write_all(
            r#"
impl Feedbacks {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            "#
            .as_bytes(),
        )?;

        self.file.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    if let Some(code_fb) = &code.feedback {
                        let params = &code_fb
                            .parameters
                            .iter()
                            .map(|param| {
                                [
                                    match param.data_type {
                                        ParameterType::Bool => "&[if ".as_bytes(),
                                        ParameterType::Int8 | ParameterType::Uint8 => {
                                            "&[".as_bytes()
                                        }
                                        ParameterType::String => "".as_bytes(),
                                        _ => "&".as_bytes(),
                                    },
                                    param.name.as_bytes(),
                                    match param.data_type {
                                        ParameterType::Uint8 => "] as &[u8]".as_bytes(),
                                        ParameterType::Bool => " {1u8} else {0u8}]".as_bytes(),
                                        ParameterType::Uint16
                                        | ParameterType::Uint32
                                        | ParameterType::Uint64
                                        | ParameterType::Int64
                                        | ParameterType::Int32
                                        | ParameterType::Int16 => ".to_le_bytes()".as_bytes(),
                                        ParameterType::Int8 => " as u8]".as_bytes(),
                                        ParameterType::String => ".as_bytes(), &[0u8]".as_bytes(),
                                    },
                                ]
                                .concat()
                            })
                            .collect::<Vec<Vec<u8>>>()
                            .join(",".as_bytes());

                        Some(
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
                            .concat(),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<Vec<u8>>>()
                .join(",\n\t\t\t".as_bytes())
                .as_slice(),
        )?;

        self.file.write_all(
            r#"
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        match bytes[0] {
            "#
            .as_bytes(),
        )?;

        self.file.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    if let Some(code_fb) = &code.feedback {
                        let mut start = 1;
                        let params_parsing = &code_fb
                            .parameters
                            .iter()
                            .map(|param| {
                                let data_len = match param.data_type {
                                    ParameterType::Bool
                                    | ParameterType::Uint8
                                    | ParameterType::Int8 => 1,
                                    ParameterType::Uint16 | ParameterType::Int16 => 2,
                                    ParameterType::Uint32 | ParameterType::Int32 => 4,
                                    ParameterType::Uint64 | ParameterType::Int64 => 8,
                                    ParameterType::String => -1,
                                };
                                let res = [
                                    format!("let {} = ", param.name).as_bytes(),
                                    // Front content
                                    match param.data_type {
                                        ParameterType::Uint16
                                        | ParameterType::Uint32
                                        | ParameterType::Uint64
                                        | ParameterType::Int64
                                        | ParameterType::Int32
                                        | ParameterType::Int16 => format!(
                                            "{}::from_le_bytes(",
                                            param.data_type.to_rust_type_string()
                                        ),
                                        ParameterType::String => "String::from_utf8(".into(),
                                        _ => "".into(),
                                    }
                                    .as_bytes(),
                                    match data_len {
                                        -1 => format!("bytes[{start}..]"),
                                        1 => format!("bytes[{start}]"),
                                        _ => format!("bytes[{}..{}]", start, start + data_len),
                                    }
                                    .as_bytes(),
                                    // Back content
                                    match param.data_type {
                                        ParameterType::Uint16
                                        | ParameterType::Uint32
                                        | ParameterType::Uint64
                                        | ParameterType::Int64
                                        | ParameterType::Int32
                                        | ParameterType::Int16 => {
                                            ".try_into().unwrap())".as_bytes()
                                        }
                                        ParameterType::String => ".to_vec()).unwrap()".as_bytes(),
                                        ParameterType::Int8 => " as i8".as_bytes(),
                                        ParameterType::Bool => " != 0u8".as_bytes(),
                                        _ => "".as_bytes(),
                                    },
                                    ";\n".as_bytes(),
                                ]
                                .concat();
                                start += data_len;
                                res
                            })
                            .collect::<Vec<Vec<u8>>>()
                            .join("\t\t\t\t".as_bytes());

                        Some(format!(
                            r#"{}u8 => {{
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
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t")
                .as_bytes(),
        )?;

        self.file.write_all(
            r#",
    _ => Err(())
        }
    }
}

impl Instructions {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
    "#
            .as_bytes(),
        )?;

        self.file.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    if let Some(code_fb) = &code.instruction {
                        let params = &code_fb
                            .parameters
                            .iter()
                            .map(|param| {
                                [
                                    match param.data_type {
                                        ParameterType::Int8 | ParameterType::Uint8 => {
                                            "&[".as_bytes()
                                        }
                                        ParameterType::Bool => "&[if ".as_bytes(),
                                        ParameterType::String => "".as_bytes(),
                                        _ => "&".as_bytes(),
                                    },
                                    param.name.as_bytes(),
                                    match param.data_type {
                                        ParameterType::Uint8 => "] as &[u8]".as_bytes(),
                                        ParameterType::Bool => " {1u8} else {0u8}]".as_bytes(),
                                        ParameterType::Uint16
                                        | ParameterType::Uint32
                                        | ParameterType::Uint64
                                        | ParameterType::Int64
                                        | ParameterType::Int32
                                        | ParameterType::Int16 => ".to_le_bytes()".as_bytes(),
                                        ParameterType::Int8 => " as u8]".as_bytes(),
                                        ParameterType::String => ".as_bytes()".as_bytes(),
                                    },
                                ]
                                .concat()
                            })
                            .collect::<Vec<Vec<u8>>>()
                            .join(",".as_bytes());

                        Some(
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
                            .concat(),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<Vec<u8>>>()
                .join(",\n\t\t\t".as_bytes())
                .as_slice(),
        )?;

        self.file.write_all(
            r#"
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        match bytes[0] {
            "#
            .as_bytes(),
        )?;

        self.file.write_all(
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    if let Some(code_fb) = &code.instruction {
                        let mut start = 1;
                        let params_parsing = &code_fb
                            .parameters
                            .iter()
                            .map(|param| {
                                let data_len = match param.data_type {
                                    ParameterType::Bool
                                    | ParameterType::Uint8
                                    | ParameterType::Int8 => 1,
                                    ParameterType::Uint16 | ParameterType::Int16 => 2,
                                    ParameterType::Uint32 | ParameterType::Int32 => 4,
                                    ParameterType::Uint64 | ParameterType::Int64 => 8,
                                    ParameterType::String => -1,
                                };
                                let res = [
                                    format!("let {} = ", param.name).as_bytes(),
                                    // Front content
                                    match param.data_type {
                                        ParameterType::Uint16
                                        | ParameterType::Uint32
                                        | ParameterType::Uint64
                                        | ParameterType::Int64
                                        | ParameterType::Int32
                                        | ParameterType::Int16 => format!(
                                            "{}::from_le_bytes(",
                                            param.data_type.to_rust_type_string()
                                        ),
                                        ParameterType::String => "String::from_utf8(".into(),
                                        _ => "".into(),
                                    }
                                    .as_bytes(),
                                    match data_len {
                                        -1 => format!("bytes[{start}..]"),
                                        1 => format!("bytes[{start}]",),
                                        _ => format!("bytes[{}..{}]", start, start + data_len),
                                    }
                                    .as_bytes(),
                                    // Back content
                                    match param.data_type {
                                        ParameterType::Uint16
                                        | ParameterType::Uint32
                                        | ParameterType::Uint64
                                        | ParameterType::Int64
                                        | ParameterType::Int32
                                        | ParameterType::Int16 => {
                                            ".try_into().unwrap())".as_bytes()
                                        }
                                        ParameterType::String => ".to_vec()).unwrap()".as_bytes(),
                                        ParameterType::Int8 => " as i8".as_bytes(),
                                        ParameterType::Bool => " != 0u8".as_bytes(),
                                        _ => "".as_bytes(),
                                    },
                                    ";\n".as_bytes(),
                                ]
                                .concat();
                                start += data_len;
                                res
                            })
                            .collect::<Vec<Vec<u8>>>()
                            .join("\t\t\t\t".as_bytes());

                        Some(format!(
                            r#"{}u8 => {{
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
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t")
                .as_bytes(),
        )?;

        self.file.write_all(
            r#",
    _ => Err(())
        }
    }
}
"#
            .as_bytes(),
        )?;

        Ok(())
    }
}

impl RustFileGenerator {
    fn map_inst_feed_rust(inst: &yaml_parser::InstFeedback) -> String {
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
        Ok(Self { file })
    }
}
