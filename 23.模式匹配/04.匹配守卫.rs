fn main() {
    //  匹配守卫
    let s1 = Some(9);
    match s1 {
        Some(x) if x % 2 == 0 => println!("x 是偶数"),
        Some(x) if x == 9 => println!("x 是 9"),
        Some(x) => println!("x 是基数"),
        None => println!("match None"),
    }

    let num = 3;
    let flag = true;
    match num {
        3 | 4 | 5 if flag => println!("num in 3,4,5 and y is true"),
        _ => println!("match other"),
    }

    // @绑定
    enum Msg {
        Hello { id: i32 },
    }
    let m1 = Msg::Hello { id: 11 };
    match m1 {
        Msg::Hello {
            id: id_rename @ 3..7,
        } => println!("id_rename is {id_rename}"),
        Msg::Hello { id: 10..20 } => {
            println!("match 10..20")
        }
        Msg::Hello { id } => println!("id is {id}"),
    }
}
