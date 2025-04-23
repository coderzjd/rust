fn main() {
    let arr = [3; 5];
    for item in arr {
        println!("item is {}", item)
    }
    println!("{}", arr.len());
    let arr = arr.into_iter().enumerate();
    for (item, index) in arr {
        println!("item is {},index is {}", item, index)
    }
    for item in 1..=4 {
        println!("item1=4  is {}", item)
    }
    for item in 1..4 {
        println!("item1..4 is {}", item)
    }
}
