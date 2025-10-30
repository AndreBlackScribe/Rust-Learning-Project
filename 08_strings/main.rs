fn main() {
    // Task: String manipulation
    let mut message = String::from("Rust");
    message.push_str(" is");
    message.push(' ');
    message.push_str("awesome!");
    println!("Final message: {}", message);
    println!("Length: {}", message.len());

    // Challenge: Check if a string is a palindrome
    let word = "radar";
    println!("Is '{}' a palindrome? {}", word, is_palindrome(word));

    // Bonus Challenge: Count vowels
    let sample = "Rustacean";
    println!("Vowel count in '{}': {}", sample, count_vowels(sample));
}

fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}