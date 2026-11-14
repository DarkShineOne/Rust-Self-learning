#![allow(unused_variables)]

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CommandError {
    #[error("input is empty")]
    EmptyInput,
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("rating is missing")]
    MissingRating,
    #[error("title is missing")]
    MissingTitle,
    #[error("invalid rating: {0}")]
    InvalidRating(String),
    #[error("id is missing")]
    MissingId,
    #[error("invalid id: {0}")]
    InvalidId(String),
    #[error("unexpected arguments: {0}")]
    UnexpectedArguments(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedbackCommand {
    Create { title: String, rating: u8 },
    Resolve { id: u64 },
    Reject { id: u64 },
}

/// Разбирает целое 1..=5, сохраняя исходный текст в ошибке.
pub fn parse_rating(input: &str) -> Result<u8, CommandError> {
    todo!("преобразовать ошибку parse и проверить диапазон")
}

/// Разбирает ненулевой u64.
pub fn parse_positive_id(input: &str) -> Result<u64, CommandError> {
    todo!("проверить синтаксис и положительный диапазон")
}

/// Разбирает create <rating> <title>, resolve <id> или reject <id>.
pub fn parse_command(input: &str) -> Result<FeedbackCommand, CommandError> {
    todo!("разобрать команду и распространить ошибки через ?")
}

/// Разбирает всю последовательность или останавливается на первой ошибке.
pub fn parse_commands(inputs: &[&str]) -> Result<Vec<FeedbackCommand>, CommandError> {
    todo!("использовать итераторы, collect и Result")
}
