#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_variables_output() {
        let result = format_variables(42, 3.14, true, 'R');
        assert_eq!(result, "Values -> int: 42, float: 3.14, bool: true, char: R");
        // Expected output: "Values -> int: 42, float: 3.14, bool: true, char: R"
    }

    #[test]
    fn test_inferred_types() {
        let inferred_int = 100;
        let inferred_float = 2.718;
        let inferred_bool = false;
        let inferred_char = '🦀';

        // These assertions confirm the types by using type-specific operations
        assert_eq!(inferred_int + 1, 101); // i32
        assert!((inferred_float - 2.0) > 0.0); // f64
        assert_eq!(inferred_bool, false); // bool
        assert_eq!(inferred_char, '🦀'); // char

        // Expected output:
        // Inferred types: 100, 2.718, false, 🦀
    }
}