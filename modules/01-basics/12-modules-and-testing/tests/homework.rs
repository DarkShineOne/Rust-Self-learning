use basics_12_modules::{Feedback, MemoryStore, create_feedback};

#[test]
fn public_api_creates_and_returns_feedback() {
    let mut store = MemoryStore::new();
    create_feedback(&mut store, 1, "  Useful course  ", 5).unwrap();
    assert_eq!(store.len(), 1);
    assert_eq!(
        store.get(1),
        Some(&Feedback::new(1, "Useful course", 5).unwrap())
    );
}

#[test]
fn domain_validation_does_not_change_store() {
    let mut store = MemoryStore::new();
    assert_eq!(
        create_feedback(&mut store, 0, "Title", 5),
        Err("invalid id".into())
    );
    assert_eq!(
        create_feedback(&mut store, 1, " ", 5),
        Err("title must not be empty".into())
    );
    assert_eq!(
        create_feedback(&mut store, 1, "Title", 6),
        Err("invalid rating".into())
    );
    assert!(store.is_empty());
}

#[test]
fn duplicate_id_does_not_replace_original() {
    let mut store = MemoryStore::new();
    create_feedback(&mut store, 1, "Original", 5).unwrap();
    assert_eq!(
        create_feedback(&mut store, 1, "Replacement", 1),
        Err("duplicate id".into())
    );
    assert_eq!(store.get(1).unwrap().title, "Original");
    assert_eq!(store.get(1).unwrap().rating, 5);
}
