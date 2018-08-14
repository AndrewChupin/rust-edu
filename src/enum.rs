enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn simple_enum() {
    let msg: Message = Message::Move { x: 3, y: 4 };

    // Inner enum
    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };


    match msg {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(a, b, c) => println!("ChangeColor {}", a),
        Message::Move { x: one, .. } if one > 21 => println!("Move {}", one),
        Message::Write => println!("Write"),
    };
}