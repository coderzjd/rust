use std::{future::Future, time::Duration};

use trpl::Either;

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(1)).await;
            "slow 结束了"
        };
        match timeout(slow, Duration::from_secs(5)).await {
            Ok(msg) => print!("运行成功输出 {}", msg),
            Err(time) => println!("运行时间大于 {:?} 秒", time.as_secs()),
        }
    })
}
async fn timeout<T: Future>(f1: T, max_time: Duration) -> Result<T::Output, Duration> {
    match trpl::race(f1, trpl::sleep(max_time)).await {
        Either::Left(x) => Ok(x),
        Either::Right(_) => Err(max_time),
    }
}

// 这一节没看懂，后面再看
// https://www.bilibili.com/video/BV1L4dGYuENZ?spm_id_from=333.788.videopod.sections&vd_source=cdf0a2c729dffbe88fe326838d204382
