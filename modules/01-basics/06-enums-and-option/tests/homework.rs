use basics_06_enums::{FeedbackCategory, FeedbackStatus, describe_rating};

#[test]
fn labels_cover_every_status() {
    assert_eq!(FeedbackStatus::New.label(), "new");
    assert_eq!(FeedbackStatus::InReview.label(), "in_review");
    assert_eq!(FeedbackStatus::Resolved.label(), "resolved");
    assert_eq!(FeedbackStatus::Rejected.label(), "rejected");
}

#[test]
fn same_status_is_idempotent() {
    for status in [
        FeedbackStatus::New,
        FeedbackStatus::InReview,
        FeedbackStatus::Resolved,
        FeedbackStatus::Rejected,
    ] {
        assert!(status.can_transition_to(status));
    }
}

#[test]
fn allows_only_defined_workflow_edges() {
    use FeedbackStatus::{InReview, New, Rejected, Resolved};
    assert!(New.can_transition_to(InReview));
    assert!(New.can_transition_to(Rejected));
    assert!(!New.can_transition_to(Resolved));
    assert!(InReview.can_transition_to(Resolved));
    assert!(InReview.can_transition_to(Rejected));
    assert!(!InReview.can_transition_to(New));
    assert!(!Resolved.can_transition_to(InReview));
    assert!(!Rejected.can_transition_to(New));
}

#[test]
fn parses_exact_category_codes() {
    assert_eq!(
        FeedbackCategory::from_code("bug"),
        Some(FeedbackCategory::Bug)
    );
    assert_eq!(
        FeedbackCategory::from_code("idea"),
        Some(FeedbackCategory::Idea)
    );
    assert_eq!(
        FeedbackCategory::from_code("question"),
        Some(FeedbackCategory::Question)
    );
    assert_eq!(
        FeedbackCategory::from_code("praise"),
        Some(FeedbackCategory::Praise)
    );
}

#[test]
fn rejects_noncanonical_category_codes() {
    for code in ["", "Bug", " bug", "other"] {
        assert_eq!(FeedbackCategory::from_code(code), None);
    }
}

#[test]
fn describes_rating_option() {
    assert_eq!(describe_rating(Some(1)), "Rating: 1/5");
    assert_eq!(describe_rating(Some(5)), "Rating: 5/5");
    assert_eq!(describe_rating(None), "No rating");
    assert_eq!(describe_rating(Some(0)), "Invalid rating");
    assert_eq!(describe_rating(Some(6)), "Invalid rating");
}
