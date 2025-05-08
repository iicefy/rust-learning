pub fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // ❌ Error: s1 no longer valid

    println!("s2 = {}", s2);
}

pub fn borrowing_example() {
    let s1 = String::from("Rust");
    print_length(&s1); // pass by reference
    println!("s1 is still valid: {}", s1);
}

fn print_length(s: &String) {
    println!("Length of '{}' is {}", s, s.len());
}

pub fn mutable_borrowing_example() {
    let mut s = String::from("Hello");
    append_world(&mut s);
    println!("{}", s);
}

fn append_world(s: &mut String) {
    s.push_str(", world!");
}
