#![allow(unused_variables, dead_code)]

pub mod domain;
pub mod storage;

pub use domain::Feedback;
pub use storage::MemoryStore;

/// Создаёт валидную запись и добавляет её в хранилище.
///
/// ```no_run
/// use basics_12_modules::{MemoryStore, create_feedback};
/// let mut store = MemoryStore::new();
/// create_feedback(&mut store, 1, "Title", 5).unwrap();
/// assert_eq!(store.len(), 1);
/// ```
pub fn create_feedback(
    store: &mut MemoryStore,
    id: u64,
    title: &str,
    rating: u8,
) -> Result<(), String> {
    todo!("скомпоновать domain::Feedback::new и storage::MemoryStore::add")
}
