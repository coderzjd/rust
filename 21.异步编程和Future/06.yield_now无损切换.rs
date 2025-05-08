use std::time::{Duration, Instant};

fn main() {
    trpl::run(async {
        let f1 = async {
            let on_ns = Duration::from_nanos(1);
            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::sleep(on_ns).await
                }
            }
            .await;
            let dur = Instant::now() - start;
            println!("f1 间隔时间为{:?} 秒", dur.as_secs_f32())
        };
        let f2 = async {
            let start = Instant::now();
            for _ in 1..1000 {
                trpl::yield_now().await
            }
            let dur = Instant::now() - start;
            println!("f2 间隔时间为{:?} 秒", dur.as_secs_f32())
        };
        trpl::join!(f1, f2)
    });
}
