fn main() {
    // Task: Define and use a struct
    let person = Person {
        name: String::from("Andrei"),
        age: 30,
    };
    println!("Name: {}, Age: {}", person.name, person.age);

    // Challenge: Call greeting method
    println!("{}", person.greet());

    // Bonus Challenge: Check if adult
    println!("Is adult? {}", person.is_adult());
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old.", self.name, self.age)
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}