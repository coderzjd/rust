fn main() {
    let arr = [1; 5];
    let a1 = &arr[1..];
    println!("a1 is {:?}", a1);

    let a1 = &arr[1..2];
    println!("a1 is {:?}", a1);
    let a1 = &arr[1..=2];
    println!("a1 is {:?}", a1);
    let a1 = &arr[..];
    println!("a1 is {:?}", a1);
    let a1 = &arr[..3];
    println!("a1 is {:?}", a1);
}
