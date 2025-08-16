include!(concat!(env!("OUT_DIR"), "/test_output.rs"));

fn match_buffers(expected_frame: &[u8], buffer: &Vec<u8>) {
    assert_eq!(
        expected_frame.len(),
        buffer.len(),
        "Frame length should match expected frame length"
    );
    assert_eq!(
        expected_frame,
        &buffer[..expected_frame.len()],
        "Buffer content should match expected frame"
    );
}

#[test]
fn generate_sleepin_fb() {
    let frame = Feedbacks::SleepPin { success: true };

    let result = frame.to_bytes();

    let expected_frame = [3u8, 1];

    match_buffers(&expected_frame, &result);
}

#[test]
fn generate_sleepin_inst() {
    let frame = Instructions::SleepPin {
        pre_sleep_time: 100,
        wake_pin_active_state: false,
    };

    let result = frame.to_bytes();
    let expected_frame = [0x03u8, 100, 0, 0];

    match_buffers(&expected_frame, &result);
}

#[test]
fn parse_sleepin_fb() {
    let frame = [3u8, 1];
    let decoded = Feedbacks::from_bytes(&frame).expect("Frame decoding should have worked");
    let expected = Feedbacks::SleepPin { success: true };

    assert_eq!(
        decoded, expected,
        "Decoded structure should match expectation"
    );
}

#[test]
fn parse_sleepin_inst() {
    let frame = [3u8, 100, 0, 1];
    let decoded = Instructions::from_bytes(&frame).expect("Frame decoding should have worked");
    let expected = Instructions::SleepPin {
        pre_sleep_time: 100,
        wake_pin_active_state: true,
    };

    assert_eq!(
        decoded, expected,
        "Decoded structure should match expectation"
    );
}

#[test]
fn generate_sleeptime_fb() {
    let frame = Feedbacks::SleepTime { feedback: 8 };
    let encoded = frame.to_bytes();
    let expected = [4u8, 8];

    match_buffers(&expected, &encoded);
}

#[test]
fn generate_sleeptime_inst() {
    let frame = Instructions::SleepTime {
        pre_sleep_time: 1000,
        duration: 700,
    };
    let encoded = frame.to_bytes();
    let expected = [4u8, 0xe8, 3, 0xbc, 2, 0, 0];

    match_buffers(&expected, &encoded);
}

#[test]
fn parse_sleeptime_fb() {
    let frame = [4u8, 50];
    let expected = Feedbacks::SleepTime { feedback: 50 };
    let decoded = Feedbacks::from_bytes(&frame).expect("Frame decoding should have worked");

    assert_eq!(expected, decoded, "Decoded frame should match expectation");
}

#[test]
fn parse_sleeptime_inst() {
    let frame = [4u8, 0xbc, 2, 0xe8, 3, 0, 0];
    let expected = Instructions::SleepTime {
        pre_sleep_time: 700,
        duration: 1000,
    };
    let decoded = Instructions::from_bytes(&frame).expect("Frame decoding should have worked");

    assert_eq!(expected, decoded, "Decoded frame should match expectation");
}

#[test]
fn generate_getversion_feedback() {
    let frame = Feedbacks::GetVersion {
        major: 1,
        minor: 0,
        patch: 99,
    };
    let encoded = frame.to_bytes();
    let expected = [0u8, 1, 0, 99];

    match_buffers(&expected, &encoded);
}

#[test]
fn generate_getversion_inst() {
    let frame = Instructions::GetVersion {};
    let encoded = frame.to_bytes();
    let expected = [0u8];

    match_buffers(&expected, &encoded);
}

#[test]
fn parse_getversion_fb() {
    let frame = [0, 50, 1, 9];
    let expected = Feedbacks::GetVersion {
        major: 50,
        minor: 1,
        patch: 9,
    };
    let decoded = Feedbacks::from_bytes(&frame).expect("Frame decoding should have worked");

    assert_eq!(expected, decoded, "Decoded frame should match expectation");
}

#[test]
fn parse_getversion_inst() {
    let frame = [0];
    let expected = Instructions::GetVersion {};
    let decoded = Instructions::from_bytes(&frame).expect("Frame decoding should have worked");

    assert_eq!(expected, decoded, "Decoded frame should match expectation");
}
