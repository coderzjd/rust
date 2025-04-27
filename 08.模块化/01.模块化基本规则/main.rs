fn main() {
    // 1、crate分类
    // binary crate可执行的，需要main函数
    // library crate 没有main函数，定义一些功能用于共享使用

    // 2、crate root: 编译的入口文件
    // binary crate入口： src/main.rs
    // library crate入口： src/lib.rs

    // 3、package：由一个或者多个crate构成
    // cargo.toml为描述文件： 类似于package.json
    // package规则：
    //      1、可能有多个binary crate
    //      2、最多只能一个library crate
    //      3、至少一个crate

    // 4、创建binary crate: cargon new my_create
    // 5、创建library crate: cargon new my_create --lib

    // 6、模块（相当于js的模块）
    // 1、使用mod声明
    // 2、可以嵌套声明子模块
    // 3、路径（path）
    // 4、访问控制使用:public和private
    //      1、所有模块默认为private,通过pub关键字声明后外部可用
    // 5、使用use和路径引入模块
    //      1、存在相对路径引入和绝对路径引入 super,self
    //      2、可以使用as给模块改名称 use std::result as stdResult
    //      3、pub use:命名重新导出
    // 6、结构体导出需要声明字段为pub
}
