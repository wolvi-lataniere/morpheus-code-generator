use crate::file_generator::FrameType;
use crate::file_generator::LanguageModel;
use crate::yaml_parser;

pub struct CppLanguageModel {
    pub headerfile_name: Option<String>,
}

impl LanguageModel for CppLanguageModel {
    fn custom_footer(&self) -> String {
        if self.headerfile_name.is_none() {
            r#"

// External parsing functions
extern int parse_feedback_frame(char* buffer, int len, Feedbacks* code, void **parameters);
extern int parse_instruction_frame(char* buffer, int len, Instructions* code, void **parameters);"#
                .to_string()
        } else {
            String::new()
        }
    }

    fn custom_includes(&self) -> String {
        if let Some(header_filename) = &self.headerfile_name {
            format!(
                r#"#include "{header_filename}"
    "#
            )
        } else {
            String::new()
        }
    }

    fn implement_feedbacks(&self, codes: &yaml_parser::CodesFile) -> String {
        [
            self.implement_feedbacks_builders(codes),
            self.implement_feedbacks_decoders(codes),
            self.implement_feedback_frames_dispatch(codes),
        ]
        .join("")
    }

    fn implement_instructions(&self, codes: &yaml_parser::CodesFile) -> String {
        [
            self.implement_instructions_builders(codes),
            self.implement_instructions_decoders(codes),
            self.implement_instruction_frames_dispatch(codes),
        ]
        .join("")
    }

    fn declare_instructions(&self, codes: &yaml_parser::CodesFile) -> String {
        [
            self.declare_instructions_enum(codes),
            self.declare_instructions_structures(codes),
            self.declare_instructions_functions(codes),
        ]
        .join("")
    }

    fn declare_feedbacks(&self, codes: &yaml_parser::CodesFile) -> String {
        [
            self.declare_feedback_enum(codes),
            self.declare_feedback_structures(codes),
            self.declare_feedbacks_functions(codes),
        ]
        .join("")
    }
}

impl CppLanguageModel {
    fn declare_feedback_enum(&self, parameters: &crate::CodesFile) -> String {
        [
            self.enumerations_header("Feedbacks"),
            self.get_formatted_enumeration_codes(&parameters.get_feedbacks(), "FB"),
            self.enumerations_footer("Feedbacks"),
        ]
        .join("")
    }

    fn declare_feedback_structures(&self, parameters: &crate::CodesFile) -> String {
        parameters
            .get_feedbacks()
            .iter()
            .map(|(_, name, params)| self.create_parameters_structure(name, params, "fb"))
            .collect::<String>()
    }

    fn declare_instructions_enum(&self, parameters: &crate::CodesFile) -> String {
        [
            self.enumerations_header("Instructions"),
            self.get_formatted_enumeration_codes(&parameters.get_instructions(), "INST"),
            self.enumerations_footer("Instructions"),
        ]
        .join("")
    }

    fn declare_instructions_structures(&self, parameters: &crate::CodesFile) -> String {
        parameters
            .get_instructions()
            .iter()
            .map(|(_, name, params)| self.create_parameters_structure(name, params, "inst"))
            .collect::<String>()
    }

    fn get_formatted_enumeration_codes(
        &self,
        parameters: &Vec<(u32, String, yaml_parser::InstFeedback)>,
        prefix: &str,
    ) -> String {
        parameters
            .iter()
            .map(|(instruction_code, name, _)| {
                format!("{}_{} = {}", prefix, name.to_uppercase(), instruction_code)
            })
            .collect::<Vec<String>>()
            .join(",\n\t")
    }

    fn enumerations_header(&self, enumeration_name: &str) -> String {
        format!(
            r#"
// {} Codes list
typedef enum __{}_enum {{
    "#,
            enumeration_name,
            enumeration_name.to_lowercase()
        )
    }

    fn enumerations_footer(&self, enumeration_name: &str) -> String {
        format!(
            r#"
}} {enumeration_name};

"#
        )
    }

    fn create_parameters_structure(
        &self,
        name: &str,
        parameters: &yaml_parser::InstFeedback,
        struct_type: &str,
    ) -> String {
        format!(
            r#"
struct s_{}_{}_params {{
    {}
}};
"#,
            struct_type,
            name.to_lowercase(),
            parameters
                .parameters
                .iter()
                .map(|p| { p.c_parameter_definition_with_comment() })
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }

    fn declare_feedbacks_functions(&self, codes: &crate::CodesFile) -> String {
        codes
            .get_feedbacks()
            .iter()
            .map(|(_, name, _)| self.declare_build_function(name, "feedback", "fb"))
            .collect::<String>()
    }

    fn declare_instructions_functions(&self, codes: &crate::CodesFile) -> String {
        codes
            .get_instructions()
            .iter()
            .map(|(_, name, _)| self.declare_build_function(name, "instruction", "inst"))
            .collect::<String>()
    }

    fn declare_build_function(&self, name: &str, inst: &str, short: &str) -> String {
        format!(
            r#"
int build_{inst}_{}_frame(char* buffer, int *len, struct s_{short}_{}_params* parameters);
"#,
            name.to_lowercase(),
            name.to_lowercase()
        )
    }
}

struct WriteFrameBuilder<'a> {
    builder_type: FrameType,
    key: String,
    name: &'a str,
    instruction: &'a yaml_parser::InstFeedback,
}

impl<'a> WriteFrameBuilder<'a> {
    pub fn new(
        builder_type: FrameType,
        name: &'a str,
        instruction: &'a yaml_parser::InstFeedback,
    ) -> Self {
        let builder_type_upper = builder_type.short().to_uppercase();
        let instruction_name_upper = name.to_uppercase();
        let key = format!("{builder_type_upper}_{instruction_name_upper}");
        WriteFrameBuilder {
            builder_type,
            key,
            name,
            instruction,
        }
    }

    pub fn build_frame(&self) -> String {
        let lowercase_name = self.name.to_lowercase();
        let key = &self.key;
        let type_long = self.builder_type.long();
        let type_short = self.builder_type.short();

        [
            format!(
                r#"
int build_{type_long}_{lowercase_name}_frame(char* buffer, int *len, struct s_{type_short}_{lowercase_name}_params* parameters)
{{

    if ((buffer == NULL) || (len == NULL) || (parameters == NULL))
      return -1;
    
    buffer_slice slice = {{.head=buffer, .len= (size_t) *len, .valid = true}};

    if (*len > 0) buffer[0] = {key};
    else return -1;

    slice = move_buffer_slice(slice, 1);
"#)
            ,

            self.instruction
                .parameters
                .iter()
                .map(|p| {
                    format!(
                        "\t\tslice = write_{}_to_buffer(slice, parameters->{});\n",
                        p.data_type.to_rust_type_string(),
                        p.name
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),

            r#"
    if (!slice.valid) {{
      return -1;
    }}
    *len = (slice.head - buffer);

    return 0;
}
        "#.to_string()].join("")
    }

    pub fn build_frame_parser(&self) -> String {
        let lowercase_name = self.name.to_lowercase();
        let key = &self.key;
        let inst = self.instruction;
        let type_long = self.builder_type.long();
        let type_short = self.builder_type.short();

        [    format!(
                r#"
int parse_{type_long}_{lowercase_name}_frame(char* buffer, int len, struct s_{type_short}_{lowercase_name}_params* parameters)
{{
    const size_t p_size = sizeof(struct s_{type_short}_{lowercase_name}_params);
    if (buffer == NULL)
        return -1;

    if ((p_size > 0) && (parameters == NULL))
        return -1;
        
    // Check the code
    if (buffer[0] != {key}) return -1;

    buffer_slice slice = {{.head=buffer, .len=(size_t)len, .valid=true}};
    slice = move_buffer_slice(slice, 1);
"#
            ),

            inst.parameters
                .iter()
                .map(|p| {
                    format!(
                        "\t\tparameters->{} = parse_{}_from_buffer(&slice);",
                        p.name,
                        p.data_type.to_rust_type_string()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),

            format!(
                r#"
    if (!slice.valid) {{
       {}return -1;
    }}
    return 0;
}}
        "#,
                inst.parameters
                    .iter()
                    .filter_map(|p| if yaml_parser::ParameterType::String == p.data_type {
                        Some(format!(
                            r#"
        if (parameters->{} != NULL) {{
            free(parameters->{});
        }}"#,
                            p.name, p.name
                        ))
                    } else {
                        None
                    })
                    .collect::<Vec<String>>()
                    .join("\n\t\t\t")
            )
        ].join("")
    }

    pub fn build_dispatch_case(&self) -> String {
        let lowercase_name = self.name.to_lowercase();
        let uppercase_name = self.name.to_uppercase();
        let frametype_upper = self.builder_type.short().to_uppercase();
        let frametype_lower = self.builder_type.short();
        let frametype_long = self.builder_type.long();
        let key = format!("{frametype_upper}_{uppercase_name}");

        format!(
            r#"
        case {key}:
            {{
                const size_t psize = sizeof(struct s_{frametype_lower}_{lowercase_name}_params);
                *parameters = k_malloc(psize);
                memset(*parameters, 0, psize);
                *code = {frametype_upper}_{uppercase_name};
                int result =  parse_{frametype_long}_{lowercase_name}_frame(buffer, len, (struct s_{frametype_lower}_{lowercase_name}_params*)*parameters);
                if (result < 0) {{
                   k_free(*parameters);
                   *parameters=NULL;
                }}
                return result;
            }}
        "#
        )
    }
}

impl CppLanguageModel {
    fn write_instruction_frame_builder(
        &self,
        _key: u32,
        name: &str,
        instruction: &yaml_parser::InstFeedback,
    ) -> String {
        WriteFrameBuilder::new(FrameType::Instruction, name, instruction).build_frame()
    }

    fn write_feedback_frame_builder(
        &self,
        _key: u32,
        name: &str,
        instruction: &yaml_parser::InstFeedback,
    ) -> String {
        WriteFrameBuilder::new(FrameType::Feedback, name, instruction).build_frame()
    }

    fn write_feedback_frame_parser(
        &self,
        _key: u32,
        name: &str,
        fb: &yaml_parser::InstFeedback,
    ) -> String {
        WriteFrameBuilder::new(FrameType::Feedback, name, fb).build_frame_parser()
    }

    fn write_instruction_frame_parser(
        &self,
        _key: u32,
        name: &str,
        inst: &yaml_parser::InstFeedback,
    ) -> String {
        WriteFrameBuilder::new(FrameType::Instruction, name, inst).build_frame_parser()
    }

    fn write_frames_dispatch(
        &self,
        builder_type: FrameType,
        instructions: Vec<(u32, String, yaml_parser::InstFeedback)>,
    ) -> String {
        let dispatch_type = builder_type.long();
        let struct_name = builder_type.struct_name();
        // To the frame decoding hub
        [
            format!(
                r#"
int parse_{dispatch_type}_frame(char* buffer, int len, {struct_name}* code, void **parameters)
{{
    if ((buffer == NULL) || (code == NULL) || (parameters == NULL))
        return -3;

    switch (buffer[0])
    {{
    "#
            ),
            instructions
                .iter()
                .map(|(_k, name, code)| {
                    WriteFrameBuilder::new(builder_type, name, code).build_dispatch_case()
                })
                .collect::<Vec<String>>()
                .join(""),
            r#"
    default: 
        return -2;
    }
}

    "#
            .to_string(),
        ]
        .join("")
    }

    fn implement_feedback_frames_dispatch(&self, codes: &yaml_parser::CodesFile) -> String {
        self.write_frames_dispatch(FrameType::Feedback, codes.get_feedbacks())
    }

    fn implement_instruction_frames_dispatch(&self, codes: &yaml_parser::CodesFile) -> String {
        self.write_frames_dispatch(FrameType::Instruction, codes.get_instructions())
    }

    fn implement_feedbacks_builders(&self, codes: &crate::CodesFile) -> String {
        codes
            .get_feedbacks()
            .iter()
            .map(|(k, name, inst)| self.write_feedback_frame_builder(*k, name, inst))
            .collect::<Vec<String>>()
            .join("")
    }

    fn implement_instructions_builders(&self, codes: &crate::CodesFile) -> String {
        codes
            .get_instructions()
            .iter()
            .map(|(k, name, inst)| self.write_instruction_frame_builder(*k, name, inst))
            .collect::<Vec<String>>()
            .join("")
    }

    fn implement_feedbacks_decoders(&self, codes: &crate::CodesFile) -> String {
        codes
            .get_feedbacks()
            .iter()
            .map(|(k, name, fb)| self.write_feedback_frame_parser(*k, name, fb))
            .collect::<Vec<String>>()
            .join("")
    }

    fn implement_instructions_decoders(&self, codes: &crate::CodesFile) -> String {
        codes
            .get_instructions()
            .iter()
            .map(|(k, name, inst)| self.write_instruction_frame_parser(*k, name, inst))
            .collect::<Vec<String>>()
            .join("")
    }
}
