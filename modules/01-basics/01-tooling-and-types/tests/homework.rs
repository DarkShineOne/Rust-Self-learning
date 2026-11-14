use basics_01_tooling::{average_rating, celsius_to_fahrenheit, fahrenheit_to_celsius};

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-10,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn converts_freezing_and_boiling_points() {
    assert_close(celsius_to_fahrenheit(0.0), 32.0);
    assert_close(celsius_to_fahrenheit(100.0), 212.0);
    assert_close(fahrenheit_to_celsius(32.0), 0.0);
    assert_close(fahrenheit_to_celsius(212.0), 100.0);
}

#[test]
fn converts_negative_temperature() {
    assert_close(celsius_to_fahrenheit(-40.0), -40.0);
    assert_close(fahrenheit_to_celsius(-40.0), -40.0);
}

#[test]
fn calculates_fractional_average() {
    assert_close(average_rating(&[5, 4, 4]), 13.0 / 3.0);
    assert_close(average_rating(&[1, 2]), 1.5);
}

#[test]
fn empty_ratings_have_zero_average() {
    assert_close(average_rating(&[]), 0.0);
}

#[test]
fn large_number_of_ratings_does_not_overflow_u8() {
    let ratings = [5_u8; 300];
    assert_close(average_rating(&ratings), 5.0);
}
