use crate::file_generator::LanguageModel;
use crate::yaml_parser::{self, ParameterType};

impl yaml_parser::ParameterType {
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

pub struct RustLanguageModel {}

impl LanguageModel for RustLanguageModel {
    fn custom_includes(&self) -> String {
        String::new()
    }

    fn custom_footer(&self) -> String {
        String::new()
    }

    fn declare_instructions(&self, codes: &crate::CodesFile) -> String {
        [
            r#"
#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
pub enum Instructions {
    "#
            .to_string(),
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
                .join("\n\t"),
            r#"
}


"#
            .to_string(),
        ]
        .join("")
    }

    fn declare_feedbacks(&self, codes: &crate::CodesFile) -> String {
        [
            r#"
#[derive(PartialEq, Eq, Clone, Serialize, Debug)]
pub enum Feedbacks {
    "#
            .to_string(),
            codes
                .codes
                .iter()
                .filter_map(|(_k, code)| {
                    code.feedback.clone().map(|inst| {
                        format!(
                            "{}{},    // {}",
                            code.name,
                            Self::map_instfeedback_list_and_type(&inst),
                            inst.description
                        )
                    })
                })
                .collect::<Vec<String>>()
                .join("\n\t"),
            r#"
}



"#
            .to_string(),
        ]
        .join("")
    }

    fn implement_feedbacks(&self, codes: &crate::CodesFile) -> String {
        [
            r#"
impl Feedbacks {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            "#
            .to_string(),
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.feedback
                        .clone()
                        .map(|code_fb| build_frame_from_fields(id, code, &code_fb))
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t"),
            r#"
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, TypesEnumError> {
        match bytes[0] {
            "#
            .to_string(),
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.feedback
                        .clone()
                        .map(|code_fb| parse_frame_to_fields(id, code, &code_fb))
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t"),
            r#",
    _ => Err(TypesEnumError::UnknownCode)
        }
    }
}
"#
            .to_string(),
        ]
        .join("")
    }

    fn implement_instructions(&self, codes: &crate::CodesFile) -> String {
        [
            r#"

impl Instructions {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
    "#
            .to_string(),
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.instruction
                        .clone()
                        .map(|code_fb| build_frame_from_fields(id, code, &code_fb))
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t"),
            r#"
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, TypesEnumError> {
        match bytes[0] {
            "#
            .to_string(),
            codes
                .codes
                .iter()
                .filter_map(|(&id, code)| {
                    code.instruction
                        .clone()
                        .map(|code_fb| parse_frame_to_fields(id, code, &code_fb))
                })
                .collect::<Vec<String>>()
                .join(",\n\t\t\t"),
            r#",
    _ => Err(TypesEnumError::UnknownCode)
        }
    }
}
"#
            .to_string(),
        ]
        .join("")
    }
}

impl RustLanguageModel {
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

fn build_frame_from_fields(id: u32, code: &crate::Codes, code_fb: &crate::InstFeedback) -> String {
    let params = code_fb
        .parameters
        .iter()
        .map(|param| {
            format!(
                "TypesEnum::{}({}).write_to_buffer().as_slice()",
                param.data_type.to_typesenum_name(),
                param.name
            )
        })
        .collect::<Vec<String>>()
        .join(",");

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
        ),
        params,
        "].concat()".to_string(),
    ]
    .concat()
}
