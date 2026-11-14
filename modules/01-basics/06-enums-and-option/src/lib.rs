#![allow(unused_variables)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackStatus {
    New,
    InReview,
    Resolved,
    Rejected,
}

impl FeedbackStatus {
    /// Проверяет разрешённый переход. Повторный переход в тот же статус разрешён.
    pub fn can_transition_to(self, next: Self) -> bool {
        todo!("описать машину состояний через match")
    }

    /// Возвращает стабильное API-имя статуса в snake_case.
    pub fn label(self) -> &'static str {
        todo!("сопоставить каждый вариант со строкой")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackCategory {
    Bug,
    Idea,
    Question,
    Praise,
}

impl FeedbackCategory {
    /// Разбирает строчный код bug, idea, question или praise.
    pub fn from_code(code: &str) -> Option<Self> {
        todo!("вернуть None для неизвестного кода")
    }
}

/// Форматирует оценку, отсутствие оценки или ошибку диапазона.
pub fn describe_rating(rating: Option<u8>) -> String {
    todo!("разобрать Option и диапазон 1..=5")
}
