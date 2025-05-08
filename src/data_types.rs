pub fn show_scalar_types() {
    let int_val: i32 = -10;
    let float_val: f64 = 3.14;
    let is_active: bool = true;
    let letter: char = 'R';

    println!("Integer: {}", int_val);
    println!("Float: {}", float_val);
    println!("Boolean: {}", is_active);
    println!("Char: {}", letter);
}

pub fn show_compound_types() {
    // Tuple
    let person: (&str, i32) = ("Alice", 30);
    println!("Name: {}, Age: {}", person.0, person.1);

    // Array
    let scores: [i32; 3] = [95, 88, 76];
    println!("First score: {}", scores[0]);
}
