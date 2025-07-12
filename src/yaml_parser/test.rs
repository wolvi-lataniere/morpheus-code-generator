use crate::yaml_parser::*;

#[test]
fn parsing_fails() -> Result<(), String> {
    let from : String = "toto".into();
    if let Ok(_result) = InstFeedbackParameterType::try_from(from)
    {
        Err("Should panic with \"toto\"".into())
    }
    else {
        Ok(())
    }
}

fn construct_from_string_and_match_type(stringType: &str, expect: InstFeedbackParameterType) -> Result<(), String> {
    let parsed_type = construct_parameter_from_char_array(stringType);
    types_match_or_error(expect, parsed_type)
}

fn construct_parameter_from_char_array(name: &str) -> Result<InstFeedbackParameterType, String> {
    let from: String = name.into();
    InstFeedbackParameterType::try_from(from)
}

fn types_match_or_error(expect: InstFeedbackParameterType, actual: Result<InstFeedbackParameterType, String>) -> Result<(), String> {
    match actual {
        Ok(actual_type) if (actual_type == expect) => Ok(()),
        Ok(other_type) => Err(format!("Wrong type: {:?} instead of {:?}", other_type.toString(), expect.toString())),
        Err(message) => Err(message)
    } 
}

#[test]
fn type_uint8_doesnt_give_uint16() -> Result<(), String> {
    if let Ok(()) = construct_from_string_and_match_type("uint8", InstFeedbackParameterType::Uint16) {
        Err("Matching function must be wrong".into())
    } else {
        Ok(())
    }
}

fn construct_and_match_type_from_array(input_iterator: Box<dyn Iterator<Item=&str>>, expect: InstFeedbackParameterType) ->
Result<(), String> {
    let test_results = input_iterator.map(|type_string| construct_from_string_and_match_type(type_string, expect));
    test_results.reduce(|previous, current| previous.and(current) ).unwrap()
}


#[test]
fn type_uint8_gives_uint8() -> Result<(), String> {
    let input_types_iter = ["uint8", "Uint8", "U8", "u8", "byte"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint8)
}

#[test]
fn type_uint16_gives_uint16() -> Result<(), String> {
    let input_types_iter = ["uint16", "Uint16", "U16", "u16"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint16)
}

#[test]
fn type_uint32_gives_uint32() -> Result<(), String> {
    let input_types_iter = ["uint32", "Uint32", "U32", "u32"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint32)
}

#[test]
fn type_uint64_gives_uint64() -> Result<(), String> {
    let input_types_iter = ["uint64", "Uint64", "U64", "u64"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Uint64)
}

#[test]
fn type_int8_gives_int8() -> Result<(), String> {
    let input_types_iter = ["int8", "Int8", "I8", "i8"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int8)
}

#[test]
fn type_int16_gives_int16() -> Result<(), String> {
    let input_types_iter = ["int16", "Int16", "I16", "i16"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int16)
}

#[test]
fn type_int32_gives_int32() -> Result<(), String> {
    let input_types_iter = ["int32", "Int32", "I32", "i32"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int32)
}

#[test]
fn type_int64_gives_int64() -> Result<(), String> {
    let input_types_iter = ["int64", "Int64", "I64", "i64"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Int64)
}
#[test]
fn type_string_gives_string() -> Result<(), String> {
    let input_types_iter = ["str", "string", "String", "Str"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::String)
}
#[test]
fn type_boolean_gives_boolean() -> Result<(), String> {
    let input_types_iter = ["bool", "Bool", "Boolean", "boolean"].into_iter();
    construct_and_match_type_from_array(Box::new(input_types_iter), InstFeedbackParameterType::Bool)
}

