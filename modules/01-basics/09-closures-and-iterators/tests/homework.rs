use basics_09_iterators::{
    Feedback, average_by_status, count_matching, positive_titles, top_ratings,
};

fn feedback(id: u64, rating: u8, resolved: bool) -> Feedback {
    Feedback {
        id,
        title: format!("Feedback {id}"),
        rating,
        resolved,
    }
}

fn sample() -> Vec<Feedback> {
    vec![
        feedback(1, 5, false),
        feedback(2, 2, true),
        feedback(3, 4, true),
        feedback(4, 3, false),
    ]
}

#[test]
fn positive_titles_borrow_and_preserve_order() {
    let items = sample();
    let titles = positive_titles(&items);
    assert_eq!(titles, ["Feedback 1", "Feedback 3"]);
    assert_eq!(titles[0].as_ptr(), items[0].title.as_ptr());
}

#[test]
fn average_separates_statuses_and_empty_groups() {
    let items = sample();
    assert_eq!(average_by_status(&items, false), Some(4.0));
    assert_eq!(average_by_status(&items, true), Some(3.0));
    assert_eq!(average_by_status(&[], true), None);
}

#[test]
fn top_ratings_sorts_and_limits() {
    let items = sample();
    assert_eq!(top_ratings(&items, 3), [5, 4, 3]);
    assert_eq!(top_ratings(&items, 20), [5, 4, 3, 2]);
    assert_eq!(top_ratings(&items, 0), Vec::<u8>::new());
    assert_eq!(items[0].rating, 5, "input order/content stays unchanged");
}

#[test]
fn accepts_stateful_fn_mut_closure() {
    let items = sample();
    let mut calls = 0;
    let count = count_matching(&items, |item| {
        calls += 1;
        item.id % 2 == 0
    });
    assert_eq!(count, 2);
    assert_eq!(calls, 4);
}
