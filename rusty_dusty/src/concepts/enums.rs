enum Message {
    Quit,                         
    Move { x: i32, y: i32 }, 
    Write(String),            
    ChangeColor(u8, u8, u8),
}

#[repr(u8)]
enum Status {
    Active = 1,
    Inactive = 0,
    Pending = 2,
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color: RGB({},{},{})", r, g, b),
        }
    }
}

pub fn main() {
    println!("Enums");

    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Move {  x, y } => msg.call(),
        _ => println!("Other variant"),
    }
    
    // if let syntax.
    if let Message::Write(text) = msg {
        println!("Text message: {}", text);  
    }
    
    // while let syntax.
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            optional = None;
        } else {
            optional = Some(i + 1);
        }
    }
    println!()
}