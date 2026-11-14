use basics_03_ownership::{duplicate_title, normalize_title, take_and_return};

#[test]
fn normalizes_without_changing_inner_whitespace() {
    let title = String::from("  Новая Идея  ");
    let result = normalize_title(title);
    assert_eq!(result, "НОВАЯ ИДЕЯ");
    // title перемещён: повторное использование здесь не скомпилировалось бы.
}

#[test]
fn returns_ownership_and_utf8_byte_length() {
    let (text, length) = take_and_return(String::from("ёж"));
    assert_eq!(text, "ёж");
    assert_eq!(length, 4);
}

#[test]
fn returns_unchanged_empty_string() {
    let (text, length) = take_and_return(String::new());
    assert_eq!(text, "");
    assert_eq!(length, 0);
}

#[test]
fn duplicate_creates_independent_strings() {
    let original = String::from("idea");
    let (mut first, second) = duplicate_title(original);
    first.push_str(" updated");
    assert_eq!(first, "idea updated");
    assert_eq!(second, "idea");
}

#[test]
fn clone_can_be_used_when_original_is_still_needed() {
    let original = String::from("feedback");
    let copy = original.clone();
    let (returned, _) = take_and_return(copy);
    assert_eq!(original, returned);
}
