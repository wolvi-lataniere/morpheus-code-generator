mod parsing {
    use crate::yaml_parser::types::*;
    use std::convert::TryFrom;
    use std::iter::Iterator;

    #[test]
    fn parsing_fails() -> Result<(), String> {
        let from: String = "toto".into();
        ParameterType::try_from(from).unwrap_err();
        Ok(())
    }

    fn construct_from_string_and_match_type(
        stringType: &str,
        expect: ParameterType,
    ) -> Result<(), String> {
        let parsed_type = construct_parameter_from_char_array(stringType);
        types_match_or_error(expect, parsed_type)
    }

    fn construct_parameter_from_char_array(name: &str) -> Result<ParameterType, String> {
        let from: String = name.into();
        ParameterType::try_from(from)
    }

    fn types_match_or_error(
        expect: ParameterType,
        actual: Result<ParameterType, String>,
    ) -> Result<(), String> {
        match actual {
            Ok(actual_type) if (actual_type == expect) => Ok(()),
            Ok(other_type) => Err(format!(
                "Wrong type: {:?} instead of {:?}",
                other_type.to_string(),
                expect.to_string()
            )),
            Err(message) => Err(message),
        }
    }

    #[test]
    fn type_uint8_doesnt_give_uint16() -> Result<(), String> {
        construct_from_string_and_match_type("uint8", ParameterType::Uint16).unwrap_err();
        Ok(())
    }

    fn construct_and_match_type_from_array(
        input_iterator: Box<dyn Iterator<Item = &str>>,
        expect: ParameterType,
    ) -> Result<(), String> {
        let test_results = input_iterator
            .map(|type_string| construct_from_string_and_match_type(type_string, expect));
        test_results
            .reduce(|previous, current| previous.and(current))
            .unwrap()
    }

    #[test]
    fn type_uint8_gives_uint8() -> Result<(), String> {
        let input_types_iter = ["uint8", "Uint8", "U8", "u8", "byte"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Uint8)
    }

    #[test]
    fn type_uint16_gives_uint16() -> Result<(), String> {
        let input_types_iter = ["uint16", "Uint16", "U16", "u16"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Uint16)
    }

    #[test]
    fn type_uint32_gives_uint32() -> Result<(), String> {
        let input_types_iter = ["uint32", "Uint32", "U32", "u32"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Uint32)
    }

    #[test]
    fn type_uint64_gives_uint64() -> Result<(), String> {
        let input_types_iter = ["uint64", "Uint64", "U64", "u64"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Uint64)
    }

    #[test]
    fn type_int8_gives_int8() -> Result<(), String> {
        let input_types_iter = ["int8", "Int8", "I8", "i8"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Int8)
    }

    #[test]
    fn type_int16_gives_int16() -> Result<(), String> {
        let input_types_iter = ["int16", "Int16", "I16", "i16"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Int16)
    }

    #[test]
    fn type_int32_gives_int32() -> Result<(), String> {
        let input_types_iter = ["int32", "Int32", "I32", "i32"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Int32)
    }

    #[test]
    fn type_int64_gives_int64() -> Result<(), String> {
        let input_types_iter = ["int64", "Int64", "I64", "i64"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Int64)
    }

    #[test]
    fn type_string_gives_string() -> Result<(), String> {
        let input_types_iter = ["str", "string", "String", "Str"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::String)
    }

    #[test]
    fn type_boolean_gives_boolean() -> Result<(), String> {
        let input_types_iter = ["bool", "Bool", "Boolean", "boolean"].into_iter();
        construct_and_match_type_from_array(Box::new(input_types_iter), ParameterType::Bool)
    }
}

mod display {
    use crate::yaml_parser::*;

    #[test]
    fn uint8_displays_uint8() -> Result<(), String> {
        assert_eq!("Uint8", ParameterType::Uint8.to_string());
        Ok(())
    }

    #[test]
    fn uint16_displays_uint16() -> Result<(), String> {
        assert_eq!("Uint16", ParameterType::Uint16.to_string());
        Ok(())
    }

    #[test]
    fn uint32_displays_uint32() -> Result<(), String> {
        assert_eq!("Uint32", ParameterType::Uint32.to_string());
        Ok(())
    }

    #[test]
    fn uint64_displays_uint64() -> Result<(), String> {
        assert_eq!("Uint64", ParameterType::Uint64.to_string());
        Ok(())
    }
    #[test]
    fn int8_displays_int8() -> Result<(), String> {
        assert_eq!("Int8", ParameterType::Int8.to_string());
        Ok(())
    }

    #[test]
    fn int16_displays_int16() -> Result<(), String> {
        assert_eq!("Int16", ParameterType::Int16.to_string());
        Ok(())
    }

    #[test]
    fn int32_displays_int32() -> Result<(), String> {
        assert_eq!("Int32", ParameterType::Int32.to_string());
        Ok(())
    }

    #[test]
    fn int64_displays_int64() -> Result<(), String> {
        assert_eq!("Int64", ParameterType::Int64.to_string());
        Ok(())
    }

    #[test]
    fn Bool_displays_bool() -> Result<(), String> {
        assert_eq!("Bool", ParameterType::Bool.to_string());
        Ok(())
    }

    #[test]
    fn string_displays_string() -> Result<(), String> {
        assert_eq!("String", ParameterType::String.to_string());
        Ok(())
    }
}

mod size {
    use crate::yaml_parser::types::ParameterType;

    #[test]
    fn uint8_size_is_1_byte() {
        assert_eq!(1, ParameterType::Uint8.size());
    }

    #[test]
    fn int8_size_is_1_byte() {
        assert_eq!(1, ParameterType::Int8.size());
    }

    #[test]
    fn uint16_size_is_2_byte() {
        assert_eq!(2, ParameterType::Uint16.size());
    }

    #[test]
    fn int16_size_is_2_byte() {
        assert_eq!(2, ParameterType::Int16.size());
    }

    #[test]
    fn uint32_size_is_4_byte() {
        assert_eq!(4, ParameterType::Uint32.size());
    }

    #[test]
    fn int32_size_is_4_byte() {
        assert_eq!(4, ParameterType::Int32.size());
    }

    #[test]
    fn uint64_size_is_8_byte() {
        assert_eq!(8, ParameterType::Uint64.size());
    }

    #[test]
    fn int64_size_is_8_byte() {
        assert_eq!(8, ParameterType::Int64.size());
    }

    #[test]
    fn bool_size_is_1_byte() {
        assert_eq!(1, ParameterType::Bool.size());
    }

    #[test]
    fn string_size_is_unknown_bytes_long() {
        assert_eq!(0, ParameterType::String.size());
    }
}

mod decoder {
    use crate::yaml_parser::types::ParameterType;
    use crate::yaml_parser::*;
    use serde_yaml;

    #[test]
    fn parse_normal_yaml_file() -> Result<(), String> {
        let test_input = "
codes:
  0x00:
    name: \"First message\"
    instruction:
      description: \"First instruction\"
      parameters:
        - name: \"a_string\"
          data_type: string
          description: A string to parse
        - name: a_bool
          data_type: bool
          description: A boolean data
        - name: a_byte
          data_type: byte
          description: A byte data
";
        let parsed: CodesFile = serde_yaml::from_str(test_input).unwrap();

        assert_eq!(1, parsed.codes.len());
        assert_eq!("First message", parsed.codes[&0].name);
        assert_eq!(
            "First instruction",
            parsed.codes[&0].instruction.as_ref().unwrap().description
        );

        assert_eq!(
            3,
            parsed.codes[&0]
                .instruction
                .as_ref()
                .unwrap()
                .parameters
                .len()
        );
        assert_eq!(
            "a_string",
            parsed.codes[&0].instruction.as_ref().unwrap().parameters[0].name
        );
        assert_eq!(
            ParameterType::String,
            parsed.codes[&0].instruction.as_ref().unwrap().parameters[0].data_type
        );

        assert_eq!(
            "a_bool",
            parsed.codes[&0].instruction.as_ref().unwrap().parameters[1].name
        );
        assert_eq!(
            ParameterType::Bool,
            parsed.codes[&0].instruction.as_ref().unwrap().parameters[1].data_type
        );
        assert_eq!(
            "a_byte",
            parsed.codes[&0].instruction.as_ref().unwrap().parameters[2].name
        );
        assert_eq!(
            ParameterType::Uint8,
            parsed.codes[&0].instruction.as_ref().unwrap().parameters[2].data_type
        );
        Ok(())
    }

    #[test]
    fn parse_wrong_format_yaml_file() -> Result<(), String> {
        let test_input = "
codes:
  0x00:
    name: \"First message\"
    instruction:
      description: \"First instruction\"
      parameters:
        - name: \"a_string\"
          data_type: strings
          description: A string to parse
";
        let parsed: Result<CodesFile, serde_yaml::Error> = serde_yaml::from_str(test_input);

        assert_eq!(
            "codes.0x00.instruction.parameters[0].data_type: invalid value: string \"strings\", expected a string representing type [uXX, iXX, string or bool] at line 9 column 22",
            parsed.unwrap_err().to_string()
        );

        Ok(())
    }
}


mod parameter {
    use crate::yaml_parser::{InstFeedbackParameter, ParameterType};

    #[test]
    fn generate_parameter_c_string() -> Result<(), String> {
        let parameter = InstFeedbackParameter {
            name: "parameter_name".into(),
            description: "parameters description".into(),
            data_type: ParameterType::Uint32
        };

        assert_eq!("uint32_t parameter_name", parameter.c_parameter_definition());
        Ok(())    
    }

    #[test]
    fn generate_parameter_c_string_with_comment() -> Result<(), String> {
        let parameter = InstFeedbackParameter {
            name: "parameter_name".into(),
            description: "parameter description".into(),
            data_type: ParameterType::Uint32
        };

        assert_eq!("uint32_t parameter_name;\t// parameter description", parameter.c_parameter_definition_with_comment());
        Ok(())    
    }
}

