use basics_10_traits::{Feedback, MemoryRepository, Repository, RepositoryError, find_max_by_key};

fn feedback(id: u64, rating: u8) -> Feedback {
    Feedback {
        id,
        title: format!("Feedback {id}"),
        rating,
    }
}

#[test]
fn insert_get_and_len_work() {
    let mut repository = MemoryRepository::new();
    assert!(repository.is_empty());
    repository.insert(feedback(2, 4)).unwrap();
    repository.insert(feedback(1, 5)).unwrap();
    assert_eq!(repository.len(), 2);
    assert_eq!(repository.get(1), Some(&feedback(1, 5)));
    assert_eq!(repository.get(9), None);
}

#[test]
fn duplicate_insert_does_not_replace_original() {
    let mut repository = MemoryRepository::new();
    repository.insert(feedback(1, 5)).unwrap();
    assert_eq!(
        repository.insert(feedback(1, 1)),
        Err(RepositoryError::DuplicateId)
    );
    assert_eq!(repository.get(1).unwrap().rating, 5);
}

#[test]
fn update_replaces_only_existing_record() {
    let mut repository = MemoryRepository::new();
    repository.insert(feedback(1, 2)).unwrap();
    repository.update(feedback(1, 4)).unwrap();
    assert_eq!(repository.get(1).unwrap().rating, 4);
    assert_eq!(
        repository.update(feedback(2, 5)),
        Err(RepositoryError::NotFound)
    );
    assert_eq!(repository.len(), 1);
}

#[test]
fn delete_returns_owned_record() {
    let mut repository = MemoryRepository::new();
    repository.insert(feedback(1, 2)).unwrap();
    assert_eq!(repository.delete(1), Some(feedback(1, 2)));
    assert_eq!(repository.delete(1), None);
    assert!(repository.is_empty());
}

#[test]
fn values_are_sorted_by_id() {
    let mut repository = MemoryRepository::new();
    for id in [3, 1, 2] {
        repository.insert(feedback(id, 3)).unwrap();
    }
    let ids: Vec<u64> = repository
        .values()
        .into_iter()
        .map(|item| item.id)
        .collect();
    assert_eq!(ids, [1, 2, 3]);
}

#[test]
fn generic_max_returns_first_maximum_by_key() {
    let items = [feedback(1, 3), feedback(2, 5), feedback(3, 5)];
    let maximum = find_max_by_key(&items, |item| item.rating).unwrap();
    assert_eq!(maximum.id, 2);
    assert_eq!(find_max_by_key(&items, |item| item.id).unwrap().id, 3);
    assert_eq!(find_max_by_key(&[], |item: &Feedback| item.id), None);
}
