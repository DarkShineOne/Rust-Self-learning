use basics_11_lifetimes::{TextView, lines_containing, longest};

#[test]
fn longest_returns_one_of_inputs() {
    let left = String::from("left");
    let right = String::from("longer");
    assert_eq!(longest(&left, &right).as_ptr(), right.as_ptr());
    assert_eq!(longest("same", "four").as_ptr(), "same".as_ptr());
    assert_eq!(longest("ё", "a").as_ptr(), "ё".as_ptr());
}

#[test]
fn first_line_outlives_view_but_not_text() {
    let text = String::from("first\r\nsecond");
    let line = {
        let view = TextView::new(&text);
        view.first_line()
    };
    assert_eq!(line, "first");
    assert_eq!(line.as_ptr(), text.as_ptr());
}

#[test]
fn view_handles_empty_and_returns_inner() {
    let text = String::new();
    let view = TextView::new(&text);
    assert_eq!(view.first_line(), "");
    assert_eq!(view.into_inner().as_ptr(), text.as_ptr());
}

#[test]
fn filters_borrowed_lines() {
    let text = "Rust ownership\nweb server\nRust lifetimes\n";
    assert_eq!(
        lines_containing(text, "Rust"),
        ["Rust ownership", "Rust lifetimes"]
    );
    assert_eq!(lines_containing(text, "rust"), Vec::<&str>::new());
    assert_eq!(
        lines_containing(text, ""),
        ["Rust ownership", "web server", "Rust lifetimes"]
    );
    assert_eq!(lines_containing("", "x"), Vec::<&str>::new());
}
