fn main() {
    // 动态大小类型和Sized Trait
    // DST【动态大小的类型必须放在指针后面使用】
    // 1、大小在编译时未知
    // let s1: str = "111";
    // let s2: str = "222333";
    // 2、&str保存地址指针就可以确定下来
    let s1: &str = "111";
    let s2: &str = "222333";

    // 编译器会为泛型参数实现Sized
    fn gen1<T>(t: T) {}
    fn gen2<T: Sized>(t: T) {}
    fn gen3<T: ?Sized>(t: &T) {
        // T: ?Sized 表示T可能是动态大小
        // 所以返回值类型只能是&T表示确定大小
    }
    let x = 1;
    gen1(x);
    gen2(x);
    let str= "111";
    gen3(str);

}
