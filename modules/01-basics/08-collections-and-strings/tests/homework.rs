use std::collections::{BTreeSet, HashMap};

use basics_08_collections::{
    Category, FeedbackRecord, char_count, count_categories, index_by_id, unique_words,
};

fn record(id: u64, category: Category, text: &str) -> FeedbackRecord {
    FeedbackRecord {
        id,
        category,
        text: text.into(),
    }
}

#[test]
fn counts_categories_without_empty_defaults() {
    let records = vec![
        record(1, Category::Bug, "a"),
        record(2, Category::Idea, "b"),
        record(3, Category::Bug, "c"),
    ];
    let counts = count_categories(&records);
    assert_eq!(counts.get(&Category::Bug), Some(&2));
    assert_eq!(counts.get(&Category::Idea), Some(&1));
    assert_eq!(counts.get(&Category::Question), None);
    assert!(count_categories(&[]).is_empty());
}

#[test]
fn unique_words_normalizes_and_sorts() {
    let actual = unique_words("Rust, rust! Привет-мир; привет 123");
    let expected = BTreeSet::from([
        "123".to_owned(),
        "rust".to_owned(),
        "привет".to_owned(),
        "мир".to_owned(),
    ]);
    assert_eq!(actual, expected);
    assert!(unique_words(" , ; ").is_empty());
}

#[test]
fn index_moves_records_and_keeps_last_duplicate() {
    let records = vec![
        record(1, Category::Bug, "first"),
        record(2, Category::Idea, "second"),
        record(1, Category::Other, "last"),
    ];
    let index: HashMap<u64, FeedbackRecord> = index_by_id(records);
    assert_eq!(index.len(), 2);
    assert_eq!(index.get(&1).unwrap().text, "last");
    assert_eq!(index.get(&1).unwrap().category, Category::Other);
    assert_eq!(index.get(&2).unwrap().text, "second");
}

#[test]
fn counts_chars_not_utf8_bytes() {
    assert_eq!(char_count("abc"), 3);
    assert_eq!(char_count("ёж"), 2);
    assert_eq!(char_count("e\u{301}"), 2);
    assert_eq!(char_count(""), 0);
}
