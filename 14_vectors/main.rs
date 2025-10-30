fn main() {
    // Task: Create and modify a vector of strings
    let mut fruits = vec!["apple".to_string(), "banana".to_string()];
    fruits.push("orange".to_string());
    fruits.push("banana".to_string()); // duplicate
    fruits.remove(0); // remove "apple"

    println!("Modified vector: {:?}", fruits);

    // Challenge: Sort and deduplicate
    fruits.sort();
    fruits.dedup();
    println!("Sorted & deduplicated: {:?}", fruits);

    // Bonus Challenge: Comma-separated string
    let result = join_vector(&fruits);
    println!("Joined string: {}", result);
}

fn join_vector(v: &Vec<String>) -> String {
    v.join(", ")
}