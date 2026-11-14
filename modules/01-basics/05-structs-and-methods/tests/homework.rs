use basics_05_structs::Feedback;

#[test]
fn creates_valid_feedback_and_trims_title() {
    let feedback = Feedback::new(1, String::from("  Great docs  "), 5).unwrap();
    assert_eq!(feedback.id, 1);
    assert_eq!(feedback.title, "Great docs");
    assert_eq!(feedback.rating, 5);
    assert!(feedback.is_positive());
}

#[test]
fn rejects_invalid_fields_in_contract_order() {
    assert_eq!(
        Feedback::new(0, String::new(), 9).unwrap_err(),
        "id must be greater than zero"
    );
    assert_eq!(
        Feedback::new(1, String::from(" \t"), 9).unwrap_err(),
        "title must not be empty"
    );
    assert_eq!(
        Feedback::new(1, String::from("Title"), 0).unwrap_err(),
        "rating must be between 1 and 5"
    );
}

#[test]
fn positive_rating_has_clear_boundary() {
    let neutral = Feedback::new(1, String::from("Neutral"), 3).unwrap();
    let positive = Feedback::new(2, String::from("Good"), 4).unwrap();
    assert!(!neutral.is_positive());
    assert!(positive.is_positive());
}

#[test]
fn updates_rating_only_when_valid() {
    let mut feedback = Feedback::new(1, String::from("Title"), 2).unwrap();
    feedback.update_rating(4).unwrap();
    assert_eq!(feedback.rating, 4);
    let error = feedback.update_rating(6).unwrap_err();
    assert_eq!(error, "rating must be between 1 and 5");
    assert_eq!(feedback.rating, 4);
}

#[test]
fn into_title_moves_out_of_feedback() {
    let feedback = Feedback::new(1, String::from("Owned title"), 5).unwrap();
    assert_eq!(feedback.into_title(), "Owned title");
}
