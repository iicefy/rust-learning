// Define an enum
pub enum Status {
    Active,
    Inactive,
    Pending,
}

// Enum with associated data
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

pub fn test_status_enum() {
    let statuses = vec![Status::Active, Status::Inactive, Status::Pending];

    for user_status in statuses {
        match user_status {
            Status::Active => println!("User is active"),
            Status::Inactive => println!("User is inactive"),
            Status::Pending => println!("User is pending approval"),
        }
    }
}

pub fn test_message_enum() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        match msg {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
        }
    }
}
