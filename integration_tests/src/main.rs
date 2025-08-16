mod test_c;
mod test_rust;

#[cfg(test)]
pub fn match_buffers(expected: &[i8], buffer: &[i8], len: usize) {
    assert_eq!(
        len,
        expected.len(),
        "Buffer length should match expectation"
    );
    assert!(
        expected.eq(&buffer[..len]),
        "Buffer content should match expectation"
    );
}

fn main() {}
