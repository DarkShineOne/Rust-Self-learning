use basics_04_borrowing::{clamp_all, first_word, sum_slice};

#[test]
fn first_word_borrows_from_input() {
    let text = String::from("  Rust ownership");
    let word = first_word(&text);
    assert_eq!(word, "Rust");
    let expected_pointer = text[2..].as_ptr();
    assert_eq!(word.as_ptr(), expected_pointer);
}

#[test]
fn first_word_handles_empty_and_unicode_whitespace() {
    assert_eq!(first_word(""), "");
    assert_eq!(first_word(" \t\n"), "");
    assert_eq!(first_word("Привет\u{2003}мир"), "Привет");
}

#[test]
fn sum_uses_wide_result() {
    assert_eq!(sum_slice(&[i32::MAX, i32::MAX]), 2 * i64::from(i32::MAX));
    assert_eq!(sum_slice(&[-3, 0, 7]), 4);
    assert_eq!(sum_slice(&[]), 0);
}

#[test]
fn clamps_in_place() {
    let mut values = [-10, 0, 5, 10, 20];
    clamp_all(&mut values, 0, 10);
    assert_eq!(values, [0, 0, 5, 10, 10]);
}

#[test]
fn clamp_accepts_empty_slice() {
    let mut values: [i32; 0] = [];
    clamp_all(&mut values, -1, 1);
    assert!(values.is_empty());
}
