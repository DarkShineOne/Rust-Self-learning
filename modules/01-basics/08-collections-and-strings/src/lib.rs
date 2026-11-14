#![allow(unused_variables)]

use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    Bug,
    Idea,
    Question,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackRecord {
    pub id: u64,
    pub category: Category,
    pub text: String,
}

/// Подсчитывает записи каждой встретившейся категории.
pub fn count_categories(records: &[FeedbackRecord]) -> HashMap<Category, usize> {
    todo!("использовать entry API")
}

/// Возвращает уникальные слова в нижнем регистре, отсортированные лексикографически.
pub fn unique_words(text: &str) -> BTreeSet<String> {
    todo!("разделить по неалфавитно-цифровым Unicode символам")
}

/// Строит индекс по id. При повторном id в индексе остаётся последняя запись.
pub fn index_by_id(records: Vec<FeedbackRecord>) -> HashMap<u64, FeedbackRecord> {
    todo!("потребить Vec и переместить записи в HashMap")
}

/// Возвращает число Unicode scalar values (char), а не байтов.
pub fn char_count(text: &str) -> usize {
    todo!("посчитать char через итератор")
}
