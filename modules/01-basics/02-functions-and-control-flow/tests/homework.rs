use basics_02_control_flow::{classify_number, collatz_steps, fizz_buzz, sum_even_below};

#[test]
fn classifies_all_number_kinds() {
    assert_eq!(classify_number(-10), "negative");
    assert_eq!(classify_number(0), "zero");
    assert_eq!(classify_number(42), "positive");
}

#[test]
fn fizz_buzz_prioritizes_fifteen() {
    assert_eq!(fizz_buzz(15), "FizzBuzz");
    assert_eq!(fizz_buzz(30), "FizzBuzz");
    assert_eq!(fizz_buzz(0), "FizzBuzz");
}

#[test]
fn fizz_buzz_covers_remaining_cases() {
    assert_eq!(fizz_buzz(3), "Fizz");
    assert_eq!(fizz_buzz(5), "Buzz");
    assert_eq!(fizz_buzz(7), "7");
    assert_eq!(fizz_buzz(1), "1");
}

#[test]
fn sums_only_values_below_limit() {
    assert_eq!(sum_even_below(0), 0);
    assert_eq!(sum_even_below(1), 0);
    assert_eq!(sum_even_below(2), 0);
    assert_eq!(sum_even_below(10), 20);
    assert_eq!(sum_even_below(11), 30);
}

#[test]
fn counts_collatz_steps() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(3), 7);
    assert_eq!(collatz_steps(6), 8);
}
