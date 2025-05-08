fn main() {
    struct Car {
        width: i32,
    }
    let c1 = Car { width: 1 };
    let Car { width } = c1;
    println!("{}", width)
}
