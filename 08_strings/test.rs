#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_length() {
        let mut message = String::from("Rust");
        message.push_str(" is");
        message.push(' ');
        message.push_str("awesome!");
        assert_eq!(message.len(), 18);
        // Expected output: "Length: 18"
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("radar"));
        assert!(!is_palindrome("rust"));
        // Expected output:
        // Is 'radar' a palindrome? true
        // Is 'rust' a palindrome? false
    }

    #[test]
    fn test_count_vowels() {
        let count = count_vowels("Rustacean");
        assert_eq!(count, 4);
        // Expected output: "Vowel count in 'Rustacean': 4"
    }
}