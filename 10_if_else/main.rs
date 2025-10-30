fn main() {
    let number = 42;

    // Task: Check if the number is positive, negative, or zero
    if number > 0 {
        println!("{} is positive", number);
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("{} is zero", number);
    }

    // Challenge: Classify number by range
    if number < 10 {
        println!("{} is small", number);
    } else if number < 100 {
        println!("{} is medium", number);
    } else {
        println!("{} is large", number);
    }

    // Bonus Challenge: Use a function to describe the number
    println!("{}", describe_number(number));
}

fn describe_number(n: i32) -> String {
    if n < 0 {
        format!("{} is negative and ", n) + &classify_size(n)
    } else if n == 0 {
        "0 is zero".to_string()
    } else {
        format!("{} is positive and ", n) + &classify_size(n)
    }
}

fn classify_size(n: i32) -> String {
    if n < 10 {
        "small".to_string()
    } else if n < 100 {
        "medium".to_string()
    } else {
        "large".to_string()
    }
}