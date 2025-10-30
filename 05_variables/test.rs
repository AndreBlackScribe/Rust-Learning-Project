#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_variable_change() {
        let mut y = 10;
        y += 5;
        assert_eq!(y, 15);
        // Expected output: "Mutable y after: 15"
    }

    #[test]
    fn test_shadowing_variable() {
        let z = "100";
        let z = z.parse::<i32>().unwrap();
        assert_eq!(z, 100);
        // Expected output: "Shadowed z as integer: 100"
    }

    #[test]
    fn test_increase_score_function() {
        let mut score = 50;
        increase_score(&mut score);
        assert_eq!(score, 60);
        // Expected output: "Score after: 60"
    }
}