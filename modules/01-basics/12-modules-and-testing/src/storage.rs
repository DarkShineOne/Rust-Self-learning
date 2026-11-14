use crate::domain::Feedback;

#[derive(Debug, Default)]
pub struct MemoryStore {
    pub(crate) records: Vec<Feedback>,
}

impl MemoryStore {
    pub fn new() -> Self {
        todo!("создать пустое хранилище")
    }

    /// Добавляет запись или возвращает Err при повторном id.
    pub fn add(&mut self, feedback: Feedback) -> Result<(), String> {
        todo!("проверить уникальность до изменения records")
    }

    pub fn get(&self, id: u64) -> Option<&Feedback> {
        todo!("найти заимствованную запись")
    }

    pub fn len(&self) -> usize {
        todo!("вернуть число записей")
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
