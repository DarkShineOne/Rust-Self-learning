use basics_13_smart_pointers::{Feedback, SharedFeedbackStore, StoreError};

fn feedback(id: u64, rating: u8) -> Feedback {
    Feedback {
        id,
        title: format!("Feedback {id}"),
        rating,
    }
}

#[test]
fn clones_share_state_and_increment_arc_count() {
    let store = SharedFeedbackStore::new();
    assert_eq!(store.strong_count(), 1);
    let clone = store.clone();
    assert_eq!(store.strong_count(), 2);
    clone.insert(feedback(1, 5)).unwrap();
    assert_eq!(store.get(1).unwrap(), feedback(1, 5));
    drop(clone);
    assert_eq!(store.strong_count(), 1);
}

#[test]
fn separate_stores_do_not_share_state() {
    let first = SharedFeedbackStore::new();
    let second = SharedFeedbackStore::new();
    assert!(second.is_empty().unwrap());
    first.insert(feedback(1, 5)).unwrap();
    assert_eq!(second.get(1), Err(StoreError::NotFound));
}

#[test]
fn duplicate_insert_keeps_original() {
    let store = SharedFeedbackStore::new();
    store.insert(feedback(1, 5)).unwrap();
    assert_eq!(store.insert(feedback(1, 1)), Err(StoreError::DuplicateId));
    assert_eq!(store.get(1).unwrap().rating, 5);
}

#[test]
fn updates_only_valid_existing_rating() {
    let store = SharedFeedbackStore::new();
    store.insert(feedback(1, 2)).unwrap();
    store.update_rating(1, 4).unwrap();
    assert_eq!(store.update_rating(1, 0), Err(StoreError::InvalidRating));
    assert_eq!(store.update_rating(2, 4), Err(StoreError::NotFound));
    assert_eq!(store.get(1).unwrap().rating, 4);
    assert_eq!(store.len(), Ok(1));
}
