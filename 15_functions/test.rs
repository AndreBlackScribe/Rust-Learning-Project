#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_function() {
        assert_eq!(sum(5, 3), 8);
        // Expected output: "Sum of 5 and 3 is 8"
    }

    #[test]
    fn test_factorial_function() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(0), 1);
        // Expected output:
        // Factorial of 5 is 120
        // Factorial of 0 is 1
    }

    #[test]
    fn test_sum_and_product_function() {
        let (s, p) = sum_and_product(4, 2);
        assert_eq!(s, 6);
        assert_eq!(p, 8);
        // Expected output:
        // Sum: 6, Product: 8
    }
}