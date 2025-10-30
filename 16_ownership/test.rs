#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_vector() {
        let v = vec![1, 2];
        let modified = modify_vector(v);
        assert_eq!(modified, vec![1, 2, 100]);
    }

    #[test]
    fn test_clone_preserves_original() {
        let original = vec![10, 20];
        let cloned = original.clone();
        print_vector(cloned);
        assert_eq!(original, vec![10, 20]);
    }
}