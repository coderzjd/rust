mod user {
    pub fn foo() {}
}
mod bar;
mod foo;
mod gkd;
fn run() {
    // 1、模块查找规则
    //    1、查找当前文件(user)
    //    2、查找同一级目录的同名文件(bar)
    //    3、查找同级别的同名目录，并且包含mod.rs文件（gkd）,mod.rs声明子模块也只能跟mod.rs同级目录
    //    4、查找同一级目录的同名文件，但是foo不是入口文件，子模块只能声明在foo同级别的同名foo目录下

    user::foo();
    bar::run_bar();
    foo::run_foo_mod::run_foo();
    gkd::run_gkd();
    gkd::gkd_1::gkd_1();
}
pub fn run_lib() {
    println!("run_lib")
}
pub mod aa {
    pub fn run_aaa() {}
}
pub fn run_all() {}
pub mod cc {
    pub fn run_cc() {}
    pub mod ccc_01 {
        pub fn run_ccc_o1() {}
    }
}
pub mod dd {
    pub fn run_dd() {}
}
// pub use:命名重新导出
pub use dd::run_dd;
