include!(concat!(env!("OUT_DIR"), "/test_output.rs"));

#[test]
fn test_generate_sleepin_fb() {
    let frame = Feedbacks::SleepPin { success: true };

    let result = frame.to_bytes();

    assert_eq!(0x03, result[0], "Should generate a frame with code 0x03");
    assert_eq!(2, result.len(), "Frame length should be 2");
    assert_eq!(0x01, result[1], "Success byte should be 1");
}

#[test]
fn test_generate_sleepin_inst() {
    let frame = Instructions::SleepPin {
        pre_sleep_time: 100,
        wake_pin_active_state: false,
    };

    let result = frame.to_bytes();

    assert_eq!(0x03, result[0], "Instruction frame code should be 0x03");
    assert_eq!(4, result.len(), "Frame length should be 4 bytes");
    assert_eq!(100, result[1], "First parameter should be 100, 00");
    assert_eq!(0, result[2], "First parameter should be 100, 00");
    assert_eq!(0, result[3], "Second parameter should be 0");
}
