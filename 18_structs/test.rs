#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_method() {
        let person = Person {
            name: String::from("Alice"),
            age: 25,
        };
        let greeting = person.greet();
        assert_eq!(greeting, "Hello, my name is Alice and I am 25 years old.");
    }

    #[test]
    fn test_is_adult_method() {
        let adult = Person {
            name: String::from("Bob"),
            age: 20,
        };
        let child = Person {
            name: String::from("Charlie"),
            age: 10,
        };
        assert!(adult.is_adult());
        assert!(!child.is_adult());
    }
}