/// Переводит градусы Цельсия в градусы Фаренгейта.
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

/// Переводит градусы Фаренгейта в градусы Цельсия.
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

/// Возвращает среднюю оценку или 0.0 для пустого среза.
///
/// Каждый элемент входных данных находится в диапазоне 1..=5.
pub fn average_rating(ratings: &[u8]) -> f64 {
    let mut sum = 0.0_f64;

    if ratings.is_empty() {
        return sum;
    }

    for temp in ratings {
        sum += f64::from(*temp);
    }

    sum / (ratings.len() as f64)
}
