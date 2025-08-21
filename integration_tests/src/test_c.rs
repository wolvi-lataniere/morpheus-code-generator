#[cfg(test)]
use crate::match_buffers;
#[cfg(test)]
use std::os::raw::c_void;
#[cfg(test)]
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
impl PartialEq for s_inst_sleeppin_params {
    fn eq(&self, other: &Self) -> bool {
        self.wake_pin_active_state == other.wake_pin_active_state
            && self.pre_sleep_time == other.pre_sleep_time
    }
}

impl PartialEq for s_fb_sleeptime_params {
    fn eq(&self, other: &Self) -> bool {
        self.feedback == other.feedback
    }
}

impl PartialEq for s_inst_sleeptime_params {
    fn eq(&self, other: &Self) -> bool {
        println!(
            "{},{} == {},{}",
            self.duration, self.pre_sleep_time, other.duration, other.pre_sleep_time
        );
        self.duration == other.duration && self.pre_sleep_time == other.pre_sleep_time
    }
}

impl PartialEq for s_fb_getversion_params {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialEq for s_inst_coveragetest_params {
    fn eq(&self, other: &Self) -> bool {
        self.a_u64 == other.a_u64
            && self.a_i64 == other.a_i64
            && unsafe { strcmp(self.a_string, other.a_string) } == 0
    }
}

impl PartialEq for s_fb_coveragetest_params {
    fn eq(&self, other: &Self) -> bool {
        self.a_i8 == other.a_i8
            && self.a_i16 == other.a_i16
            && self.a_i32 == other.a_i32
            && unsafe { strcmp(self.a_string, other.a_string) } == 0
    }
}

#[test]
fn sleeping_sucessfull_frame() {
    let mut buffer = [0i8; 2];
    let mut len = buffer.len() as i32;
    let mut parameters = s_fb_sleeppin_params { success: true };
    let result =
        unsafe { build_feedback_sleeppin_frame(buffer.as_mut_ptr(), &mut len, &mut parameters) };

    let expected_frame = [0x03i8, 1];

    assert_eq!(0, result, "The generation should succeed");
    match_buffers(&expected_frame, &buffer, len as usize);
}

#[test]
fn sleeping_failing_frame() {
    let mut buffer = [0i8; 2];
    let mut len = buffer.len() as i32;
    let mut parameters = s_fb_sleeppin_params { success: false };
    let result =
        unsafe { build_feedback_sleeppin_frame(buffer.as_mut_ptr(), &mut len, &mut parameters) };

    let expected_frame = [0x03i8, 0];

    assert_eq!(0, result, "The generation should succeed");
    match_buffers(&expected_frame, &buffer, len as usize);
}

#[test]
fn sleeping_frame_too_small_buffer() {
    let mut buffer: [i8; 1] = [0; 1];
    let mut len: i32 = buffer.len() as i32;
    let mut parameters = s_fb_sleeppin_params { success: true };
    let result =
        unsafe { build_feedback_sleeppin_frame(buffer.as_mut_ptr(), &mut len, &mut parameters) };

    assert_ne!(
        0, result,
        "The generation should fail due to too small buffer"
    );
}

#[test]
fn parse_feedback_sleeping_frame() {
    let mut buffer: [i8; 2] = [0x03, 0x01];
    let mut code: u32 = 0;
    let parameters: s_fb_sleeppin_params;
    let mut ptr = ptr::null_mut::<c_void>();

    let result = unsafe {
        parse_feedback_frame(
            buffer.as_mut_ptr(),
            buffer.len() as i32,
            &mut code,
            &mut ptr,
        )
    };

    assert_eq!(0, result, "Should succeed to parse frame");
    assert_eq!(
        __feedbacks_enum_FB_SLEEPPIN, code,
        "Should have parsed a sleeping frame"
    );

    assert_ne!(ptr::null_mut(), ptr, "Should return a non-NULL pointer");
    unsafe {
        parameters = *(ptr as *mut s_fb_sleeppin_params);
    }

    assert!(
        parameters.success,
        "Should have parsed a 'true' success parameter"
    );

    unsafe {
        free(ptr);
    }
}

#[test]
fn parse_invalid_sleeping_frame_length() {
    let mut buf: [i8; 1] = [0x03];
    let mut code: u32 = 0;
    let mut ptr = ptr::null_mut::<c_void>();

    let result =
        unsafe { parse_feedback_frame(buf.as_mut_ptr(), buf.len() as i32, &mut code, &mut ptr) };

    assert_ne!(
        0, result,
        "Frame processing should fail due to wrong frame length"
    );
}

#[test]
fn parse_invalid_feedback_frame_code() {
    let mut buf = [0x50i8, 0x00, 0x00, 0x00, 0x00];
    let mut code = 0u32;
    let mut ptr = ptr::null_mut::<c_void>();

    let result =
        unsafe { parse_feedback_frame(buf.as_mut_ptr(), buf.len() as i32, &mut code, &mut ptr) };

    assert_ne!(
        0, result,
        "Frame decoding should fail for unknown frame code"
    );
}

#[test]
fn parse_invalid_instruction_frame_code() {
    let mut buf = [0x50i8, 0x00, 0x00, 0x00, 0x00];
    let mut code = 0u32;
    let mut ptr = ptr::null_mut::<c_void>();

    let result =
        unsafe { parse_instruction_frame(buf.as_mut_ptr(), buf.len() as i32, &mut code, &mut ptr) };

    assert_ne!(
        0, result,
        "Frame decoding should fail for unknown frame code"
    );
}

#[test]
fn parse_instruction_sleeping_frame() {
    let mut buf = [0x03i8, 120, 0, 1];
    let mut code = 0u32;
    let mut ptr = ptr::null_mut::<c_void>();
    let expected_struct = s_inst_sleeppin_params {
        pre_sleep_time: 120,
        wake_pin_active_state: true,
    };

    let result =
        unsafe { parse_instruction_frame(buf.as_mut_ptr(), buf.len() as i32, &mut code, &mut ptr) };

    assert_eq!(0, result, "Frame parsing should succeed");
    assert_ne!(
        ptr::null_mut::<c_void>(),
        ptr,
        "Should have allocated some pointer"
    );
    assert_eq!(
        __instructions_enum_INST_SLEEPPIN, code,
        "Instruction code should match Sleepin instruction"
    );
    let decoded_struct: s_inst_sleeppin_params = unsafe { *(ptr as *mut s_inst_sleeppin_params) };

    assert_eq!(
        expected_struct, decoded_struct,
        "Decoded structure should match the expected values"
    );
    unsafe {
        free(ptr);
    }
}

#[test]
fn build_sleepin_instruction() {
    let mut buf = [0i8; 255];
    let mut len = buf.len() as i32;
    let mut sent_frame = s_inst_sleeppin_params {
        pre_sleep_time: 84,
        wake_pin_active_state: false,
    };

    let expected_buffer = [3i8, 84, 0, 0];

    let result =
        unsafe { build_instruction_sleeppin_frame(buf.as_mut_ptr(), &mut len, &mut sent_frame) };

    assert_eq!(0, result, "Should succeed");

    match_buffers(&expected_buffer, &buf, len as usize);
}

#[test]
fn build_sleeptime_feedback() {
    let mut frame_to_send = s_fb_sleeptime_params { feedback: 0x55 };

    let mut buf = [0i8; 255];
    let mut len = buf.len() as i32;

    let expected_frame = [4i8, 0x55];

    let result =
        unsafe { build_feedback_sleeptime_frame(buf.as_mut_ptr(), &mut len, &mut frame_to_send) };

    assert_eq!(0, result, "Frame generation should have succeed");
    match_buffers(&expected_frame, &buf, len as usize);
}

#[test]
fn parse_feedback_sleeptime() {
    let mut frame_to_parse = [4i8, 0x7e];
    let mut ptr = ptr::null_mut::<c_void>();
    let mut code = 0u32;

    let result = unsafe {
        parse_feedback_frame(
            frame_to_parse.as_mut_ptr(),
            frame_to_parse.len() as i32,
            &mut code,
            &mut ptr,
        )
    };

    assert_eq!(0, result, "Frame parsing should have succeed");
    assert_ne!(ptr::null_mut(), ptr, "Should have allocated some memory");
    assert_eq!(
        __feedbacks_enum_FB_SLEEPTIME, code,
        "Code should be SleepTime feedback"
    );

    let decoded_frame = unsafe { *(ptr as *mut s_fb_sleeptime_params) };
    let expected_frame = s_fb_sleeptime_params { feedback: 0x7e };

    assert_eq!(
        expected_frame, decoded_frame,
        "Decoded frame should match expected one"
    );

    unsafe { free(ptr) };
}

#[test]
fn build_instruction_sleeptime() {
    let mut frame_to_send = s_inst_sleeptime_params {
        pre_sleep_time: 10,
        duration: 1000,
    };
    let mut buf = [0i8; 7];
    let mut len = buf.len() as i32;

    let expected_buffer = [0x04i8, 10, 0, -0x18, 0x03, 0, 0];

    let result = unsafe {
        build_instruction_sleeptime_frame(buf.as_mut_ptr(), &mut len, &mut frame_to_send)
    };

    assert_eq!(0, result, "Frame generation should have succeed");

    match_buffers(&expected_buffer, &buf, len as usize);
}

#[test]
fn parse_instruction_sleeptime() {
    let expected_decoded = s_inst_sleeptime_params {
        pre_sleep_time: 10,
        duration: 1000,
    };
    let mut buf = [0x04i8, 10, 0, -0x18, 0x03, 0, 0];
    let mut code = 0u32;
    let mut ptr = ptr::null_mut::<c_void>();

    let result =
        unsafe { parse_instruction_frame(buf.as_mut_ptr(), buf.len() as i32, &mut code, &mut ptr) };

    assert_eq!(0, result, "Frame parsing should have succeed");
    assert_eq!(
        __instructions_enum_INST_SLEEPTIME, code,
        "Code should match the expected one"
    );

    assert_ne!(ptr::null_mut(), ptr, "Should have allocated some memory ");

    let decoded_struct = unsafe { *(ptr as *mut s_inst_sleeptime_params) };
    assert_eq!(
        expected_decoded, decoded_struct,
        "Decoded frame should match expectation"
    );

    unsafe { free(ptr) };
}

#[test]
fn build_instruction_getversion() {
    let mut frame_to_send = s_inst_getversion_params { _address: 0u8 };
    let mut buf = [0i8; 5];
    let mut len = buf.len() as i32;
    let expected_buffer = [0i8];

    let result = unsafe {
        build_instruction_getversion_frame(buf.as_mut_ptr(), &mut len, &mut frame_to_send)
    };

    assert_eq!(0, result, "Frame generation should have worked");
    match_buffers(&expected_buffer, &buf, len as usize);
}

#[test]
fn parse_instruction_getversion() {
    let mut buf = [0i8];
    let mut code = 5u32;
    let mut ptr = ptr::null_mut::<c_void>();

    let result =
        unsafe { parse_instruction_frame(buf.as_mut_ptr(), buf.len() as i32, &mut code, &mut ptr) };

    assert_eq!(0, result, "Frame decoding should have worked");
    assert_ne!(ptr::null_mut(), ptr, "Should have allocated some memory");

    assert_eq!(
        __instructions_enum_INST_GETVERSION, code,
        "Should have GetVersion instruction id"
    );

    unsafe { free(ptr) };
}

#[test]
fn build_feedback_getversion() {
    let mut frame_to_send = s_fb_getversion_params {
        major: 1,
        minor: 0,
        patch: 99,
    };

    let mut buf = [0i8; 4];
    let mut len = buf.len() as i32;
    let expected_frame = [0i8, 1, 0, 99];

    let result =
        unsafe { build_feedback_getversion_frame(buf.as_mut_ptr(), &mut len, &mut frame_to_send) };

    assert_eq!(0, result, "Frame generation should have worked");
    match_buffers(&expected_frame, &buf, len as usize);
}

#[test]
fn parse_feedback_getversion() {
    let mut buf = [0i8, 1, 0, 99];
    let mut code = 6u32;
    let mut ptr = ptr::null_mut::<c_void>();

    let expected_struct = s_fb_getversion_params {
        major: 1,
        minor: 0,
        patch: 99,
    };
    let result =
        unsafe { parse_feedback_frame(buf.as_mut_ptr(), buf.len() as i32, &mut code, &mut ptr) };

    assert_eq!(0, result, "Frame parsing should have worked");
    assert_ne!(ptr::null_mut(), ptr, "Should have allocated some memory");

    assert_eq!(
        __feedbacks_enum_FB_GETVERSION, code,
        "Should have a getVersion feedback code"
    );

    let decoded_struct = unsafe { *(ptr as *mut s_fb_getversion_params) };
    assert_eq!(
        expected_struct, decoded_struct,
        "Decoded structure should match expected one"
    );
    unsafe { free(ptr) };
}

#[test]
fn generate_coverage_inst() {
    let mut the_string = "HelloMessage"
        .as_bytes()
        .iter()
        .map(|i| *i as i8)
        .collect::<Vec<i8>>();
    let mut base_struct = s_inst_coveragetest_params {
        a_u64: 125000000,
        a_i64: -15699,
        a_string: the_string.as_mut_ptr(),
    };

    let expected = [
        1u8, b'H', b'e', b'l', b'l', b'o', b'M', b'e', b's', b's', b'a', b'g', b'e', 0, 0x40, 0x59,
        0x73, 0x07, 0, 0, 0, 0, 0xad, 0xc2, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ]
    .iter()
    .map(|u| *u as i8)
    .collect::<Vec<i8>>();

    let mut buffer = [0i8; 255];
    let mut len: i32 = buffer.len() as i32;

    let result = unsafe {
        build_instruction_coveragetest_frame(buffer.as_mut_ptr(), &mut len, &mut base_struct)
    };

    assert_eq!(0, result, "Generation should success");

    match_buffers(&expected, &buffer, len as usize);
}

#[test]
fn parse_coverage_inst() {
    let mut the_string = "HelloMessage"
        .as_bytes()
        .iter()
        .map(|i| *i as i8)
        .collect::<Vec<i8>>();
    let expected_struct = s_inst_coveragetest_params {
        a_u64: 125000000,
        a_i64: -15699,
        a_string: the_string.as_mut_ptr(),
    };

    let mut encoded = [
        1u8, b'H', b'e', b'l', b'l', b'o', b'M', b'e', b's', b's', b'a', b'g', b'e', 0, 0x40, 0x59,
        0x73, 0x07, 0, 0, 0, 0, 0xad, 0xc2, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ]
    .iter()
    .map(|u| *u as i8)
    .collect::<Vec<i8>>();

    let mut code = 0u32;
    let mut ptr = ptr::null_mut::<c_void>();

    let result = unsafe {
        parse_instruction_frame(
            encoded.as_mut_ptr(),
            encoded.len() as i32,
            &mut code,
            &mut ptr,
        )
    };

    assert_eq!(0, result, "Parsing should success");
    assert_ne!(
        ptr::null_mut(),
        ptr,
        "Should have allocated some memory for the data"
    );

    let data = unsafe { *(ptr as *mut s_inst_coveragetest_params) };

    assert_eq!(expected_struct, data, "Generated structure should match");

    unsafe {
        free(data.a_string as *mut c_void);
        free(ptr);
    }
}

#[test]
fn generate_coverage_fb() {
    let mut msg_string = "This is a test string"
        .as_bytes()
        .iter()
        .map(|i| *i as i8)
        .collect::<Vec<i8>>();

    let mut input_struct = s_fb_coveragetest_params {
        a_i8: 99,
        a_i16: 1983,
        a_i32: -19488,
        a_string: msg_string.as_mut_ptr(),
    };

    let mut buffer = [0i8; 255];
    let mut len = buffer.len() as i32;

    let result = unsafe {
        build_feedback_coveragetest_frame(buffer.as_mut_ptr(), &mut len, &mut input_struct)
    };

    assert_eq!(0, result);

    let expected = [
        1u8, 99u8, 0xbf, 0x07, 0xe0, 0xb3, 0xff, 0xff, b'T', b'h', b'i', b's', b' ', b'i', b's',
        b' ', b'a', b' ', b't', b'e', b's', b't', b' ', b's', b't', b'r', b'i', b'n', b'g', 0,
    ]
    .iter()
    .map(|i| *i as i8)
    .collect::<Vec<i8>>();

    match_buffers(&expected, &buffer, len as usize);
}

#[test]
fn parse_coverage_fb() {
    let mut msg_string = "This is a test string"
        .as_bytes()
        .iter()
        .map(|i| *i as i8)
        .collect::<Vec<i8>>();

    let expected_struct = s_fb_coveragetest_params {
        a_i8: 99,
        a_i16: 1983,
        a_i32: -19488,
        a_string: msg_string.as_mut_ptr(),
    };

    let mut encoded = [
        1u8, 99u8, 0xbf, 0x07, 0xe0, 0xb3, 0xff, 0xff, b'T', b'h', b'i', b's', b' ', b'i', b's',
        b' ', b'a', b' ', b't', b'e', b's', b't', b' ', b's', b't', b'r', b'i', b'n', b'g', 0,
    ]
    .iter()
    .map(|i| *i as i8)
    .collect::<Vec<i8>>();

    let mut ptr = ptr::null_mut::<c_void>();
    let mut code = 0u32;

    let result = unsafe {
        parse_feedback_frame(
            encoded.as_mut_ptr(),
            encoded.len() as i32,
            &mut code,
            &mut ptr,
        )
    };

    assert_eq!(0, result);
    assert_eq!(__feedbacks_enum_FB_COVERAGETEST, code);
    assert_ne!(ptr::null_mut(), ptr);

    let decoded = unsafe { *(ptr as *mut s_fb_coveragetest_params) };

    assert_eq!(expected_struct, decoded);
}

#[test]
fn parse_invalid_string() {
    let mut encoded = [
        1u8, b'H', b'e', b'l', b'l', b'o', b'M', b'e', b's', b's', b'a', b'g', b'e', 0x40, 0x59,
        0x73, 0x07, 0, 0, 0, 0, 0xad, 0xc2, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ]
    .iter()
    .map(|u| *u as i8)
    .collect::<Vec<i8>>();

    let mut code = 0u32;
    let mut ptr = ptr::null_mut::<c_void>();

    let result = unsafe {
        parse_instruction_frame(
            encoded.as_mut_ptr(),
            encoded.len() as i32,
            &mut code,
            &mut ptr,
        )
    };

    assert_ne!(0, result, "Parsing should fail");
    assert_eq!(ptr::null_mut(), ptr, "Should have freed allocated memory");
}
