#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_world() {
        let mut s = String::from("Hi");
        append_world(&mut s);
        assert_eq!(s, "Hi, world!");
    }

    #[test]
    fn test_get_length() {
        let s = String::from("Hello");
        assert_eq!(get_length(&s), 5);
    }

    #[test]
    fn test_print_vector_length() {
        let v = vec![10, 20, 30];
        print_vector_length(&v); // Just prints, no assertion needed
    }
}