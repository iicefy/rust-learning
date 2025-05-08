pub fn show_variables() {
    // mut for mutable variable
    let mut name = "Ice";
    let age = 28;
    let is_active = true;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", is_active);

    // reassign mutable variable
    name = "Chawit";

    println!("Edited Name: {}", name);
}
