#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_check() {
        let number = 8;
        let is_even = number % 2 == 0;
        assert!(is_even);
        // Expected output: "Is 8 even? true"
    }

    #[test]
    fn test_divisible_by_3_and_5() {
        assert!(divisible_by_3_and_5(15));
        assert!(!divisible_by_3_and_5(10));
        // Expected output:
        // Is 15 divisible by both 3 and 5? true
        // Is 10 divisible by both 3 and 5? false
    }

    #[test]
    fn test_boolean_message() {
        assert_eq!(boolean_message(true), "Yes");
        assert_eq!(boolean_message(false), "No");
        // Expected output:
        // Boolean message: Yes
        // Boolean message: No
    }
}