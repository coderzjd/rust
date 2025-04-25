// 切片是一中fat指针:胖指针不仅存储地址还存储长度
fn main() {
    let s1 = String::from("998");
    // s1存储变量
    println!("s1 占用空间大小: {}", std::mem::size_of_val(&s1));
    // s2存储s1的指针地址
    let s2 = &s1;
    println!("s2 占用空间大小: {}", std::mem::size_of_val(&s2));
    // s3存了s1的指针地址+切片长度
    let s3 = &s1[..];
    println!("s3 占用空间大小: {}", std::mem::size_of_val(&s3));
    // s1 占用空间大小: 24
    // s2 占用空间大小: 8
    // s3 占用空间大小: 16
}
