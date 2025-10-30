#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_data_types() {
        let int_val: i32 = 42;
        let float_val: f64 = 3.14;
        let bool_val: bool = true;
        let char_val: char = 'R';

        assert_eq!(int_val, 42);
        assert!((float_val - 3.0) > 0.0);
        assert!(bool_val);
        assert_eq!(char_val, 'R');
        // Expected output:
        // Integer: 42
        // Float: 3.14
        // Boolean: true
        // Character: R
    }

    #[test]
    fn test_describe_tuple_function() {
        let t = (100, 6.28, false, '🦀');
        let description = describe_tuple(t);
        assert_eq!(description, "Tuple contains -> int: 100, float: 6.28, bool: false, char: 🦀");
        // Expected output: "Tuple contains -> int: 100, float: 6.28, bool: false, char: 🦀"
    }
}