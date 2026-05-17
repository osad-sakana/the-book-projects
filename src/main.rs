enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> String {
        match self {
            Message::Quit => String::from("Quit"),
            Message::Move { x, y } => format!("Move to ({}, {})", x, y),
            Message::Write(text) => format!("Write: {}", text),
            Message::ChangeColor(r, g, b) => format!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("{}", msg1.call());
    println!("{}", msg2.call());
    println!("{}", msg3.call());
    println!("{}", msg4.call());
}
