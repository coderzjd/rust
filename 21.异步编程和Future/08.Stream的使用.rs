// StreamExt拓展了原有的Stream这个trait，添加了next方法
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    // foo();
    bar()
}
fn foo() {
    // Stream
    // 1、就是异步版本的迭代器
    // 2、可以从迭代器创建stream
    trpl::run(async {
        let v1: [i32; 5] = [1, 2, 3, 4, 5];
        let v2 = v1.iter().map(|x| x * 2);
        let stream = trpl::stream_from_iter(v2);
        let mut filtered = stream.filter(|x| x % 3 == 0);
        while let Some(info) = filtered.next().await {
            println!("info is {info}")
        }
    });
}
fn bar() {
    // Stream可以与任意的Future组合使用
    fn get_msg() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();
        let msgs = ["a", "b", "c"];
        for msg in msgs {
            tx.send(format!("发送的消息是： '{msg}'")).unwrap();
        }
        // 把接收器转化为返回next方法的流
        ReceiverStream::new(rx)
    }
    trpl::run(async {
        let mut msg = get_msg();
        while let Some(info) = msg.next().await {
            println!("{info}")
        }
    })
}
