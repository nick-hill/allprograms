enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_av(m: Movement) {
    match m {
        Movement::Up => println!("Avatar Moving Up"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Left => println!("Avatar Moving Left"),
        Movement::Right => println!("Avatar Moving Right"),
    }
}

pub fn run() {
    let av1 = Movement::Right;
    let av2 = Movement::Left;
    let av3 = Movement::Up;
    let av4 = Movement::Down;

    move_av(av1);
    move_av(av2);
    move_av(av3);
    move_av(av4);
}
