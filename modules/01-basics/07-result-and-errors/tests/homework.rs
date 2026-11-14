use basics_07_result::{
    CommandError, FeedbackCommand, parse_command, parse_commands, parse_positive_id, parse_rating,
};

#[test]
fn parses_rating_and_preserves_invalid_text() {
    assert_eq!(parse_rating("1"), Ok(1));
    assert_eq!(parse_rating("5"), Ok(5));
    assert_eq!(
        parse_rating("0"),
        Err(CommandError::InvalidRating("0".into()))
    );
    assert_eq!(
        parse_rating("6"),
        Err(CommandError::InvalidRating("6".into()))
    );
    assert_eq!(
        parse_rating("five"),
        Err(CommandError::InvalidRating("five".into()))
    );
}

#[test]
fn parses_only_positive_ids() {
    assert_eq!(parse_positive_id("1"), Ok(1));
    assert_eq!(
        parse_positive_id("0"),
        Err(CommandError::InvalidId("0".into()))
    );
    assert_eq!(
        parse_positive_id("-1"),
        Err(CommandError::InvalidId("-1".into()))
    );
}

#[test]
fn parses_create_with_multword_title() {
    assert_eq!(
        parse_command("  create 5  Improve   API docs  "),
        Ok(FeedbackCommand::Create {
            title: "Improve   API docs".into(),
            rating: 5,
        })
    );
}

#[test]
fn parses_moderation_commands() {
    assert_eq!(
        parse_command("resolve 42"),
        Ok(FeedbackCommand::Resolve { id: 42 })
    );
    assert_eq!(
        parse_command("reject 7"),
        Ok(FeedbackCommand::Reject { id: 7 })
    );
}

#[test]
fn reports_missing_and_unknown_parts() {
    assert_eq!(parse_command(" \n"), Err(CommandError::EmptyInput));
    assert_eq!(
        parse_command("edit 1"),
        Err(CommandError::UnknownCommand("edit".into()))
    );
    assert_eq!(parse_command("create"), Err(CommandError::MissingRating));
    assert_eq!(parse_command("create 4"), Err(CommandError::MissingTitle));
    assert_eq!(
        parse_command("create 4   "),
        Err(CommandError::MissingTitle)
    );
    assert_eq!(parse_command("resolve"), Err(CommandError::MissingId));
    assert_eq!(
        parse_command("reject 2 extra words"),
        Err(CommandError::UnexpectedArguments("extra words".into()))
    );
}

#[test]
fn parses_collection_or_stops_at_first_error() {
    let commands = parse_commands(&["create 3 Average", "resolve 2"]).unwrap();
    assert_eq!(commands.len(), 2);
    assert_eq!(parse_commands(&[]), Ok(Vec::new()));
    assert_eq!(
        parse_commands(&["create 5 Valid", "resolve 0", "create 1 Unreachable"]),
        Err(CommandError::InvalidId("0".into()))
    );
}
