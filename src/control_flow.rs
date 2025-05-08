// control_flow.rs
pub fn test_if_statement() {
    let number = 7;
    if number < 10 {
        println!("Less than 10");
    } else {
        println!("10 or more");
    }
}

pub fn test_match_statement() {
    let grade = 'B';
    match grade {
        'A' => println!("Excellent!"),
        'B' => println!("Good job."),
        'C' => println!("Keep trying."),
        _ => println!("Unknown grade."),
    }
}

pub fn test_loops() {
    for i in 1..=3 {
        println!("Count: {}", i);
    }
}
