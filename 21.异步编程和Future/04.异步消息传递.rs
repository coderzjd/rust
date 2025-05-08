use std::time::Duration;

use trpl::{self};

fn main() {
    // foo();
    // bar()
    // gkd()
    jjk()
}
fn foo() {
    // 【异步运行时】会阻塞主线程
    // 异步调度不会创建新的线程：执行顺序由【异步运行时】控制
    trpl::run(async {
        // spawn_task：生成一个新的异步任务
        let handler = trpl::spawn_task(async {
            for num in 1..=10 {
                println!("新异步 num is {num}");
                trpl::sleep(Duration::from_secs(1)).await
            }
        });
        // 主异步块：执行完成之后就退出了，不会等待其他异步任务执行
        for num in 1..=5 {
            println!("主异步 num is {num}");
            trpl::sleep(Duration::from_secs(1)).await
        }
        // handler类似线程会阻塞当前异步任务
        // 主异步块完成之后，会继续等待其他异步任务执行
        handler.await.unwrap();
    });
}
fn bar() {
    trpl::run(async {
        // handler也是一个future
        let handler = trpl::spawn_task(async {
            for num in 1..=10 {
                println!("新异步 num is {num}");
                trpl::sleep(Duration::from_secs(1)).await
            }
        });
        let task = async {
            for num in 1..=5 {
                println!("主异步 num is {num}");
                trpl::sleep(Duration::from_secs(1)).await
            }
        };
        // 使用join函数调度两个future
        let res = trpl::join(handler, task).await;
        res.0.unwrap();
    });
}
fn gkd() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        // 使用move把tx的所有权转移到f1中
        // 当f1执行完毕，tx会被销毁
        // 当所有发送端被销毁，接收端退出循环
        let f1 = async move {
            for num in 1..=5 {
                tx.send(num).unwrap();
                trpl::sleep(Duration::from_secs(1)).await
            }
        };

        let f2 = async {
            while let Some(received) = rx.recv().await {
                println!("received is {}", received);
            }
            println!("接受完毕!")
        };
        trpl::join(f1, f2).await;
    })
}

fn jjk() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx_clone = tx.clone();
        let f1 = async move {
            for num in 1..=5 {
                tx.send(num).unwrap();
                trpl::sleep(Duration::from_secs(1)).await
            }
        };

        let f2 = async {
            while let Some(received) = rx.recv().await {
                println!("received is {}", received);
            }
            println!("接受完毕!")
        };
        let f3 = async move {
            for num in 10..=20 {
                tx_clone.send(num).unwrap();
                trpl::sleep(Duration::from_secs(1)).await
            }
        };
        // 多个发送端都销毁后才会停止接受
        trpl::join3(f1, f2, f3).await;
    })
}
