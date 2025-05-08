pub fn shadowing() {
    let x = 10;
    println!("x initialize to {}", x);

    let mut x = x;
    x = x * 2;

    println!("x reassign to mutable variable with same name {}", x);
}
