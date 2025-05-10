pub fn numbers() {
    //signed and unsigned
    // i = signed
    // u = unsigned
    // เลขข้างหลังคือจำนวน bit

    // default ของ rust เมื่อประกาศโดยไม่มี type annotation
    // int: i32, float: f64

    let mut a: i8 = -128;
    println!("min of i8 = {}", a);
    a = 127;
    println!("max of i8 = {}", a);

    let mut b: u8 = 0;
    println!("min of u8 = {}", b);
    b = 255;
    println!("max of u8 = {}", b);
}
