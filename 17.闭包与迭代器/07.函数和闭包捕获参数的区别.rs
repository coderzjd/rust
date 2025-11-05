// 函数传参
//    没实现 Copy 的变量 默认转移所有权（move）。
// 闭包捕获
//    编译器会 根据闭包体内“怎么用” 来决定捕获方式，规则如下：
//    只读使用 → 不可变借用（&T）
//    修改使用 → 可变借用（&mut T）
//    需要所有权（例如线程 move、再往下传、消耗掉）→ 移动（move）
//    声明闭包的那一刻（不是调用那一刻）就已经完成捕获；一旦捕获方式确定，在闭包存活期间对应借用就保持活跃——因此“声明即借用”，后面不能再做冲突借用。

fn main() {
    // 1. 函数：没 Copy 就 move
    let s = String::from("hi");
    fn_consume(s);          // 所有权进函数
    // println!("{}", s);   // 报错：已 move

    // 2. 闭包：编译器看“怎么用”
    let s = String::from("hi");
    let imm = || println!("{}", s);   // 体内只读 → 捕获 &String
    imm();
    let mut t = String::from("hi");
    let mut mut_b = || t.push('!');   // 体内写 → 捕获 &mut String
    // 下面这句在闭包存在期间非法：
   //  println!("{}", t);
    mut_b();                            // 调用无所谓，捕获早已完成
}
fn fn_consume(s : String){
    println!("{}",s)
}