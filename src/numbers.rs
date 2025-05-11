pub fn numbers() {
    //signed and unsigned
    // i = signed
    // u = unsigned
    // เลขข้างหลังคือจำนวน bit

    // default ของ rust เมื่อประกาศโดยไม่มี type annotation
    // int: i32, float: f64

    println!("min of i8 = {}", i8::MIN);
    println!("max of i8 = {}", i8::MAX);

    println!("min of u8 = {}", u8::MIN);
    println!("max of u8 = {}", u8::MAX);
}
