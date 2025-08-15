include!(concat!(env!("OUT_DIR"), "/test_output.rs"));

#[test]
fn test_generate_sleepin_fb() {
    let frame = Feedbacks::SleepPin { success: true };

    let result = frame.to_bytes();

    assert_eq!(0x03, result[0], "Should generate a frame with code 0x03");
    assert_eq!(2, result.len(), "Frame length should be 2");
    assert_eq!(0x01, result[1], "Success byte should be 1");
}
