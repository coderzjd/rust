// binary crate调用lib需要显示cargo.toml中的name字段use：name = "test01"
// 绝对路径导入
// 使用as关键字修改导入模块名称
use test01::aa as bbb;
use test01::run_lib;
// 从一个包导入多个
use test01::{aa, run_lib as ccc};
// 从一个包导入所有
use test01::*;
// 从一个模块导入另一个模块本身和子模块
use test01::cc::{self, ccc_01::run_ccc_o1};
fn main() {
    run_lib();
    m1::m1_1::run_m1_1();
    bbb::run_aaa();
    aa::run_aaa();
    ccc();
    run_all();
    cc::run_cc();
    run_ccc_o1();
    run_dd()
}

mod m1 {
    pub fn run_m1() {
        // self关键字：调用当前模块
        self::run_m22();
        // run_m3为私有模块无法访问
        // self::m3::run_m3()
    }
    pub mod m1_1 {
        pub fn run_m1_1() {
            // super关键字：子模块调用父模块
            super::run_m1();
        }
    }
    pub fn run_m22() {
        println!("run_m22")
    }
    pub mod m3 {
        fn run_m3() {}
    }
}
mod m2 {
    pub fn run_m2() {
        // 这里的super指向root模块
        super::m1::run_m1();
    }
    mod m2_1 {
        pub fn run_m2_1() {}
    }
}
