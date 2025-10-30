fn main() {
    // Task: Mutable and immutable variables
    let x = 5; // immutable
    println!("Immutable x: {}", x);

    let mut y = 10; // mutable
    println!("Mutable y before: {}", y);
    y += 5;
    println!("Mutable y after: {}", y);

    // Challenge: Shadowing
    let z = "100";
    let z = z.parse::<i32>().unwrap(); // shadowed with a new type
    println!("Shadowed z as integer: {}", z);

    // Bonus Challenge: Modify a variable via mutable reference
    let mut score = 50;
    println!("Score before: {}", score);
    increase_score(&mut score);
    println!("Score after: {}", score);
}

fn increase_score(val: &mut i32) {
    *val += 10;
}