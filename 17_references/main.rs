fn main() {
    // Task: Modify string using mutable reference
    let mut greeting = String::from("Hello");
    append_world(&mut greeting);
    println!("Modified greeting: {}", greeting);

    // Challenge: Use mutable and immutable references
    let mut message = String::from("Rust");
    {
        let len = get_length(&message); // Immutable borrow
        println!("Length of message: {}", len);
    } // Immutable borrow ends here

    {
        let msg_ref = &mut message; // Mutable borrow
        msg_ref.push_str("acean");
    }
    println!("Updated message: {}", message);

    // Bonus Challenge: Reference to vector
    let numbers = vec![1, 2, 3, 4];
    print_vector_length(&numbers);
}

fn append_world(s: &mut String) {
    s.push_str(", world!");
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn print_vector_length(v: &Vec<i32>) {
    println!("Vector length: {}", v.len());
}