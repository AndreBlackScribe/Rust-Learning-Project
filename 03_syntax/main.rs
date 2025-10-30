fn main() {
    // Task: Declare variables of different types.
    let int_val: i32 = 42;
    let float_val: f64 = 3.14;
    let bool_val: bool = true;
    let char_val: char = 'R';

    println!("Integer: {}", int_val);
    println!("Float: {}", float_val);
    println!("Boolean: {}", bool_val);
    println!("Character: {}", char_val);

    // Challenge: Call a function that returns a formatted string.
    let summary = format_variables(int_val, float_val, bool_val, char_val);
    println!("{}", summary);

    // Bonus Challenge: Type inference
    let inferred_int = 100; // inferred as i32
    let inferred_float = 2.718; // inferred as f64
    let inferred_bool = false;
    let inferred_char = '🦀';

    println!("Inferred types: {}, {}, {}, {}", inferred_int, inferred_float, inferred_bool, inferred_char);
}

fn format_variables(i: i32, f: f64, b: bool, c: char) -> String {
    format!("Values -> int: {}, float: {}, bool: {}, char: {}", i, f, b, c)
}