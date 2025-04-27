fn main() {
    enum Msg {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColro(u8, u8, u8),
    }
    impl Msg {
        fn call(&self) {
            match self {
                Msg::Quit => {
                    println!("math Quit")
                }
                Msg::Move { x, y } => {
                    println!("x is {x}, y is {y}")
                }
                Msg::Write(info) => {
                    println!(" info is {}", info)
                }
                Msg::ChangeColro(r, g, b) => {
                    println!("r is {r}, g is {g},b is {b}")
                }
            }
        }
    }
    let m1 = Msg::Write(String::from("hellow"));
    m1.call();
    let m2 = Msg::ChangeColro(0, 0, 0);
    m2.call();
    let m3 = Msg::Quit;
    m3.call();
    let m4 = Msg::Move { x: 1, y: 2 };
    m4.call();
}
