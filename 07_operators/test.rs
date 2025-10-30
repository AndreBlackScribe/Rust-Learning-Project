#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let a = 12;
        let b = 4;
        assert_eq!(a + b, 16); // Addition
        assert_eq!(a - b, 8);  // Subtraction
        assert_eq!(a * b, 48); // Multiplication
        assert_eq!(a / b, 3);  // Division
        // Expected outputs:
        // Addition: 16
        // Subtraction: 8
        // Multiplication: 48
        // Division: 3
    }

    #[test]
    fn test_modulus_and_exponentiation() {
        let a = 12;
        let b = 4;
        assert_eq!(a % b, 0); // Modulus
        assert_eq!(i32::pow(a, b), 20736); // Exponentiation
        // Expected outputs:
        // Modulus: 0
        // Exponentiation: 20736
    }

    #[test]
    fn test_calculate_function() {
        assert_eq!(calculate(12, 4, "+"), 16);
        assert_eq!(calculate(12, 4, "*"), 48);
        assert_eq!(calculate(12, 4, "^"), 20736);
        // Expected outputs:
        // Using operator '+': 16
        // Using operator '*': 48
        // Using operator '^': 20736
    }
}