fn main() {
    // Task: Basic data types
    let int_val: i32 = 42;
    let float_val: f64 = 3.14;
    let bool_val: bool = true;
    let char_val: char = 'R';

    println!("Integer: {}", int_val);
    println!("Float: {}", float_val);
    println!("Boolean: {}", bool_val);
    println!("Character: {}", char_val);

    // Challenge: Tuple with mixed types
    let mixed_tuple = (100, 6.28, false, '🦀');
    let (a, b, c, d) = mixed_tuple;
    println!("Tuple values: {}, {}, {}, {}", a, b, c, d);

    // Bonus Challenge: Use a function to describe the tuple
    println!("{}", describe_tuple(mixed_tuple));
}

fn describe_tuple(t: (i32, f64, bool, char)) -> String {
    format!("Tuple contains -> int: {}, float: {}, bool: {}, char: {}", t.0, t.1, t.2, t.3)
}