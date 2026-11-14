#![allow(unused_variables)]

#[derive(Debug, Clone, PartialEq)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub rating: u8,
    pub resolved: bool,
}

/// Возвращает заимствованные title положительных отзывов в исходном порядке.
pub fn positive_titles(feedback: &[Feedback]) -> Vec<&str> {
    todo!("filter + map + collect без клонирования строк")
}

/// Возвращает среднюю оценку выбранного статуса или None, если записей нет.
pub fn average_by_status(feedback: &[Feedback], resolved: bool) -> Option<f64> {
    todo!("агрегировать только совпавшие элементы")
}

/// Возвращает до limit оценок по убыванию. При равенстве порядок не важен.
pub fn top_ratings(feedback: &[Feedback], limit: usize) -> Vec<u8> {
    todo!("map + сортировка + take")
}

/// Считает элементы, для которых замыкание вернуло true.
pub fn count_matching(feedback: &[Feedback], predicate: impl FnMut(&Feedback) -> bool) -> usize {
    todo!("применить FnMut к заимствованным элементам")
}
