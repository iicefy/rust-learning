// Define a struct
pub struct Person {
    pub name: String,
    pub age: u32,
    pub is_active: bool,
}

// Implement methods for the struct
impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
            is_active: true,
        }
    }

    pub fn greet(&self) {
        println!(
            "Hi, my name is {} and I'm {} years old.",
            self.name, self.age
        );
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

// Function to test usage
pub fn test_structs() {
    let mut user = Person::new("Ice", 28);
    user.greet();
    println!("Active: {}", user.is_active);

    user.deactivate();
    println!("Active after deactivation: {}", user.is_active);
}
