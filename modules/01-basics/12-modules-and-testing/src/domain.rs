#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub rating: u8,
}

impl Feedback {
    /// Создаёт запись с id > 0, непустым title и rating 1..=5.
    pub fn new(id: u64, title: &str, rating: u8) -> Result<Self, String> {
        todo!("проверить доменные инварианты")
    }
}

#[cfg(test)]
mod tests {
    use super::Feedback;

    #[test]
    fn rejects_zero_rating() {
        assert!(Feedback::new(1, "Title", 0).is_err());
    }
}
