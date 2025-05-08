use trpl::{Either, Html};

fn main() {
    trpl::run(async {
        let future_one = page_title("https://kimi.moonshot.cn/");
        let future_two = page_title("https://www.bilibili.com/");
        // 类似于js的Promise.race方法
        let race_future_title = match trpl::race(future_one, future_two).await {
            Either::Left(x) => x,
            Either::Right(y) => y,
        };
        match race_future_title {
            Some(info) => println!("info is {info}"),
            None => println!("no title"),
        }

        let future_three = page_title("https://kimi.moonshot.cn/");
        let future_four = page_title("https://www.bilibili.com/");
        // 类似于js的Promise.all方法
        let info = trpl::join(future_three, future_four).await;
        println!("info is {info:?}");
        if let (Some(x), Some(y)) = info {
            println!("x is {x}");
            println!("y is {y}");
        }
       
    })
}

async fn page_title(url: &str) -> Option<String> {
    let rsp = trpl::get(url).await;
    let rsp_text = rsp.text().await;
    Html::parse(&rsp_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}
