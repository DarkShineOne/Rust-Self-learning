#![allow(unused_variables)]

/// Учебная модель обращения. Инварианты проверяются конструктором.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub rating: u8,
}

impl Feedback {
    /// Создаёт обращение с ненулевым id, непустым title и rating 1..=5.
    /// Title сохраняется без пробелов по краям.
    pub fn new(id: u64, title: String, rating: u8) -> Result<Self, String> {
        todo!("проверить инварианты и создать Feedback")
    }

    /// Возвращает true для оценки 4 или 5.
    pub fn is_positive(&self) -> bool {
        todo!("классифицировать оценку")
    }

    /// Обновляет оценку или оставляет self неизменным и возвращает Err.
    pub fn update_rating(&mut self, rating: u8) -> Result<(), String> {
        todo!("проверить значение до изменения self")
    }

    /// Потребляет обращение и возвращает его заголовок.
    pub fn into_title(self) -> String {
        todo!("переместить title из self")
    }
}
