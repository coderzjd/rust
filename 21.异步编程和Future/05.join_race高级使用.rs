use std::{
    future::Future,
    pin::{pin, Pin},
    time::Duration,
};
fn main() {
    // run_join()
    // run_join_with_pin()
    // run_race()
    run_race_with_yield_now()
}
fn run_join() {
    trpl::run(async {
        // join方法支持传入不同类型的future
        let f1 = async { 1 };
        let f2 = async { 2 };
        let info = trpl::join(f1, f2).await;
        println!("info is {info:?}");

        let f1 = async { 1 };
        let f2 = async { 2 };
        let f3 = async { "3" };
        let info2 = trpl::join3(f1, f2, f3).await;
        println!("info is {info2:?}");

        let f1 = async { 1 };
        let f2 = async { 2 };
        // join_all方法接受一个可迭代对象（用于很多数量的future）
        // 编译器会为每一个async块生成唯一的枚举，类型不一致不能放在同一个Vec内
        // let info2 = trpl::join_all(vec![f1,f2]).await;

        // Pin 是一种智能指针类型。它可以确保被指向的数据在它的生命周期内不会被移动
        // Pin 可以确保 Future 不会被移动，从而保证这些引用的有效性
        let f_f: Vec<Pin<Box<dyn Future<Output = i32>>>> = vec![Box::pin(f1), Box::pin(f2)];
        let info2 = trpl::join_all(f_f).await;
        println!("info is {info2:?}");

        // join!用于调度已知数量的future，支持传入不同类型的future
        let f1 = async { 998 };
        let f2 = async { "233" };
        let info = trpl::join!(f1, f2);
        println!("info is {info:?}");
    });
}
fn run_join_with_pin() {
    trpl::run(async {
        // 使用pin!宏固定async函数类型
        let f1 = pin!(async { 1 });
        let f2 = pin!(async { 2 });
        let ff: Vec<Pin<&mut dyn Future<Output = i32>>> = vec![f1, f2];
        let info2 = trpl::join_all(ff).await;
        println!("info is {info2:?}");
    });
}

fn run_race() {
    trpl::run(async {
        let f1 = async {
            println!("slow 111");
            // 在每一个await的地方都会暂停，把控制权交给异步运行时
            trpl::sleep(Duration::from_millis(100)).await;
            println!("slow 222")
        };
        let f2 = async {
            println!("fast 111");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("fast 222");
        };
        // race方法会先调用F1到await等待，然后调用F2到await等待，然后比较哪一个等待快就先执行哪一个
        trpl::race(f1, f2).await;
        // 传参顺序不一样，结果不一样
        // trpl::race(f2, f1).await;
    })
}
fn run_race_with_yield_now() {
    trpl::run(async {
        let f1 = async {
            println!("slow 111");
            // 使用 trpl::sleep(Duration::from_millis(100))定时器移交控制权有损耗（因为定时器至少等待1毫秒）
            // 使用yield_now可以无损移交所有权
            trpl::yield_now().await;
            println!("slow 222")
        };
        let f2 = async {
            println!("fast 111");
            trpl::yield_now().await;
            println!("fast 222");
        };
        trpl::race(f1, f2).await;
    })
}
