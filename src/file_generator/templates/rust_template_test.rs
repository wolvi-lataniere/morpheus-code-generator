#[cfg(test)]
mod test {

    use crate::file_generator::rust_template::{TypesEnum, WriteToBuffer};
    #[test]
    fn write_u8_to_buffer() {
        let value = TypesEnum::U8(9u8);
        let converted = value.write_to_buffer();

        assert_eq!(converted, &[9u8]);
    }

    #[test]
    fn write_i8_to_buffer() {
        let value = TypesEnum::I8(-9);
        let converted = value.write_to_buffer();
        let sliced = converted.as_slice();

        assert_eq!(&[-9i8 as u8], sliced);
    }

    #[test]
    fn write_u16_to_buffer() {
        let value = TypesEnum::U16(259);
        let converted = value.write_to_buffer();

        assert_eq!(converted, &[3u8, 1]);
    }

    #[test]
    fn write_i16_to_buffer() {
        let value = TypesEnum::I16(-9);
        let converted = value.write_to_buffer();
        let sliced = converted.as_slice();

        assert_eq!(&[-9i8 as u8, 0xff], sliced);
    }

    #[test]
    fn write_u32_to_buffer() {
        let value = TypesEnum::U32(259);
        let converted = value.write_to_buffer();

        assert_eq!(converted, &[3u8, 1, 0, 0]);
    }

    #[test]
    fn write_i32_to_buffer() {
        let value = TypesEnum::I32(-9);
        let converted = value.write_to_buffer();
        let sliced = converted.as_slice();

        assert_eq!(&[-9i8 as u8, 0xff, 0xff, 0xff], sliced);
    }

    #[test]
    fn write_u64_to_buffer() {
        let value = TypesEnum::U64(259);
        let converted = value.write_to_buffer();

        assert_eq!(converted, &[3u8, 1, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn write_i64_to_buffer() {
        let value = TypesEnum::I64(-9);
        let converted = value.write_to_buffer();
        let sliced = converted.as_slice();

        assert_eq!(
            &[-9i8 as u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            sliced
        );
    }

    #[test]
    fn write_bool_to_buffer() {
        let value = TypesEnum::Bool(true);
        let converted = value.write_to_buffer();
        let sliced = converted.as_slice();

        assert_eq!(&[1u8], sliced);
    }

    #[test]
    fn write_string_to_buffer() {
        let test_string = String::from("Test String");
        let value = TypesEnum::Str(test_string.clone());
        let converted = value.write_to_buffer();
        let sliced = converted.as_slice();
        let mut expected = test_string.as_bytes().to_vec();
        expected.push(0x00);

        assert_eq!(expected.as_slice(), sliced);
    }

    #[test]
    fn parse_u8_from_buffer() {
        let test_value = [99u8];
        let (result, rest) =
            TypesEnum::u8_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(0, rest.len());
        assert_eq!(99u8, result);
    }

    #[test]
    fn parse_i8_from_buffer() {
        let test_value = [-99i8 as u8, 9];
        let (result, rest) =
            TypesEnum::i8_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(1, rest.len());
        assert_eq!(-99i8, result);
    }

    #[test]
    fn parse_u16_from_buffer() {
        let test_value = [99u8, 1];
        let (result, rest) =
            TypesEnum::u16_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(0, rest.len());
        assert_eq!(99u16 + 256u16, result);
    }

    #[test]
    fn parse_i16_from_buffer() {
        let test_value = [-99i8 as u8, 0xff, 0];
        let (result, rest) =
            TypesEnum::i16_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(1, rest.len());
        assert_eq!(-99i16, result);
    }

    #[test]
    fn parse_u32_from_buffer() {
        let test_value = [99u8, 1, 0, 0];
        let (result, rest) =
            TypesEnum::u32_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(0, rest.len());
        assert_eq!(99u32 + 256u32, result);
    }

    #[test]
    fn parse_i32_from_buffer() {
        let test_value = [-99i8 as u8, 0xff, 0xff, 0xff, 0];
        let (result, rest) =
            TypesEnum::i32_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(1, rest.len());
        assert_eq!(-99i32, result);
    }

    #[test]
    fn parse_u64_from_buffer() {
        let test_value = [99u8, 1, 0, 0, 0, 0, 0, 0];
        let (result, rest) =
            TypesEnum::u64_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(0, rest.len());
        assert_eq!(99u64 + 256u64, result);
    }

    #[test]
    fn parse_i64_from_buffer() {
        let test_value = [-99i8 as u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0];
        let (result, rest) =
            TypesEnum::i64_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(1, rest.len());
        assert_eq!(-99i64, result);
    }

    #[test]
    fn parse_bool_from_buffer() {
        let test_value = [1, 0xff, 0xff, 0xff, 0];
        let (result, rest) =
            TypesEnum::bool_from_buffer(&test_value).expect("Should success parsing");

        assert_eq!(4, rest.len());
        assert!(result);
    }

    #[test]
    fn parse_string_from_buffer() {
        let expected_string = String::from("Test string");
        let mut test_buffer = expected_string.as_bytes().to_vec();
        test_buffer.append(&mut vec![0, 16, 40]);
        let (result, rest) =
            TypesEnum::string_from_buffer(test_buffer.as_slice()).expect("Should success parsing");

        assert_eq!(2, rest.len());
        assert_eq!(result, expected_string);
    }
}
