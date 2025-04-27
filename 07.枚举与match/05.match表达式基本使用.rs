fn main() {
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }
    let d1 = Direction::Right;
    // match必须匹配所有情况
    // match d1 {
    //     Direction::Left => {}
    //     Direction::Right => {}
    //     Direction::Up => {}
    // }
    match d1 {
        Direction::Left => {}
        Direction::Right => {}
        // 可以使用【_】表示匹配其他情况
        _ => {}
    }
    // match表达式匹配值
    let num = 21;
    match num {
        1 => {
            println!("num match 1")
        }
        2 => {
            println!("num match 2")
        }
        _x => {
            println!("num match other {_x}")
        }
    }
}
