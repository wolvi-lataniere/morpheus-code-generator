#[cfg(test)]
use std::os::raw::c_void;
#[cfg(test)]
use std::ptr;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn test_sleeping_sucessful_frame_file() {
    let mut buffer: [i8; 256] = [0; 256];
    let mut len: i32 = 256;
    let mut parameters = s_fb_sleeppin_params { success: true };
    let result =
        unsafe { build_feedback_sleeppin_frame(buffer.as_mut_ptr(), &mut len, &mut parameters) };

    assert_eq!(0, result, "The generation should succeed");
    assert_eq!(
        0x03, buffer[0],
        "Should have generated a frame code of 0x03"
    );
    assert_eq!(
        0x01, buffer[1],
        "Should have a success flag value of \"true\""
    );
    assert_eq!(2, len, "Should have a frame length of 2");
}

#[test]
fn test_sleeping_failing_frame_file() {
    let mut buffer: [i8; 256] = [0; 256];
    let mut len: i32 = 256;
    let mut parameters = s_fb_sleeppin_params { success: false };
    let result =
        unsafe { build_feedback_sleeppin_frame(buffer.as_mut_ptr(), &mut len, &mut parameters) };

    assert_eq!(0, result, "The generation should succeed");
    assert_eq!(
        0x03, buffer[0],
        "Should have generated a frame code of 0x03"
    );
    assert_eq!(
        0x00, buffer[1],
        "Should have a success flag value of \"true\""
    );
    assert_eq!(2, len, "Should have a frame length of 2");
}

#[test]
fn test_sleeping_frame_too_small_buffer() {
    let mut buffer: [i8; 1] = [0; 1];
    let mut len: i32 = 1;
    let mut parameters = s_fb_sleeppin_params { success: true };
    let result =
        unsafe { build_feedback_sleeppin_frame(buffer.as_mut_ptr(), &mut len, &mut parameters) };

    assert_ne!(
        0, result,
        "The generation should fail due to too small buffer"
    );
}

#[test]
fn test_parse_feedback_sleeping_frame() {
    let mut buffer: [i8; 2] = [0x03, 0x01];
    let mut code: u32 = 0;
    let len: i32 = 2;
    let parameters: s_fb_sleeppin_params;
    let mut ptr = ptr::null_mut::<c_void>();

    let result = unsafe { parse_feedback_frame(buffer.as_mut_ptr(), len, &mut code, &mut ptr) };

    assert_eq!(0, result, "Should succeed to parse frame");
    assert_eq!(
        __instructions_enum_INST_SLEEPPIN, code,
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

fn main() {}
