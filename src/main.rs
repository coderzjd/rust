fn main() {
    let m1 = "11".to_string();
    let m2 = "22".to_string();
    println!("m1: {}", m1);
    println!(" m1 的地址: {}", &m1);
    let s = format!("{},{}", m1, m2);
}
fn greet() {}
// https://www.bilibili.com/video/BV1bVS4YXEE5?spm_id_from=333.788.videopod.sections&vd_source=cdf0a2c729dffbe88fe326838d204382