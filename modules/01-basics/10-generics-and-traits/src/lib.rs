#![allow(unused_variables, dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub rating: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryError {
    DuplicateId,
    NotFound,
}

/// Абстракция хранилища с собственным ассоциированным типом ошибки.
pub trait Repository {
    type Error;

    fn insert(&mut self, feedback: Feedback) -> Result<(), Self::Error>;
    fn get(&self, id: u64) -> Option<&Feedback>;
    fn update(&mut self, feedback: Feedback) -> Result<(), Self::Error>;
    fn delete(&mut self, id: u64) -> Option<Feedback>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Default)]
pub struct MemoryRepository {
    records: HashMap<u64, Feedback>,
}

impl MemoryRepository {
    /// Создаёт пустое хранилище.
    pub fn new() -> Self {
        todo!("создать MemoryRepository")
    }

    /// Возвращает заимствованные записи, отсортированные по возрастанию id.
    pub fn values(&self) -> Vec<&Feedback> {
        todo!("собрать и детерминированно отсортировать записи")
    }
}

impl Repository for MemoryRepository {
    type Error = RepositoryError;

    fn insert(&mut self, feedback: Feedback) -> Result<(), Self::Error> {
        todo!("запретить DuplicateId")
    }

    fn get(&self, id: u64) -> Option<&Feedback> {
        todo!("найти запись по id")
    }

    fn update(&mut self, feedback: Feedback) -> Result<(), Self::Error> {
        todo!("заменить существующую запись или вернуть NotFound")
    }

    fn delete(&mut self, id: u64) -> Option<Feedback> {
        todo!("удалить и вернуть запись")
    }

    fn len(&self) -> usize {
        todo!("вернуть число записей")
    }
}

/// Возвращает первый максимальный по ключу элемент или None для пустого среза.
pub fn find_max_by_key<T, K, F>(items: &[T], key: F) -> Option<&T>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    todo!("обобщённый поиск без клонирования элементов")
}
