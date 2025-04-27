fn main() {
    enum Xiaomi {
        Redmi(i32),
        xiaomi,
    }
    enum Phone {
        Iphone,
        Other(Xiaomi),
    }
    let p1 = Phone::Other(Xiaomi::Redmi(998));
    fn match_phone(p: Phone) {
        match p {
            Phone::Iphone => {
                println!("match Iphone")
            }
            Phone::Other(info) => match info {
                Xiaomi::Redmi(x) => {
                    println!("match redmi is {x}")
                }
                Xiaomi::xiaomi => {
                    println!("match xiaomi")
                }
            },
        }
    }
    match_phone(p1);
}
