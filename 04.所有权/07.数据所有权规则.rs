fn main() {
    let mut names = (String::from("111"), String::from("222"));
    // first这个不可变借用，只会影响names和names.0的所有权
    let first = &names.0;
    // 修改names.1不影响
    names.1.push_str("998");
    // 不能修改names
    // drop(names);
    // 不能修改names.0
    // names.0.push_str("22");
    println!("dirst is {}", first);

    let mut nums = [1; 4];
    // x获得num的所有权
    let x = &mut nums[0];
    // 在x使用完成之前y不能获得num的所有权
    // let y = &nums[2];
    *x += 1;
    println!("nums is {:?}", nums);
    // println!(" y is {}", y)
}
