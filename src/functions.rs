// Define a simple function
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with a return value
pub fn square(num: i32) -> i32 {
    num * num
}

// Function with parameters and multiple statements
pub fn describe_number(num: i32) {
    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}
