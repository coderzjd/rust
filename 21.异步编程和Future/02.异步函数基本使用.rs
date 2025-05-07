use trpl::Html;

fn main() {
    // trpl::run就是一个异步运行时
    trpl::run(async {
        let title = page_title("https://kimi.moonshot.cn/").await;
        match title {
            Some(info) => println!("info is {info}"),
            None => println!("no title"),
        }
    })
}
// 1、返回Future<T>类型的函数使用await获取值
// 2、await关键字只能在标记为async函数内使用
// 3、async函数返回为Future<T>类型
// 4、async函数需要放在async块或者async函数内使用
// 5、async函数运行需要异步运行时调度
async fn page_title(url: &str) -> Option<String> {
    let rsp = trpl::get(url).await;
    let rsp_text = rsp.text().await;
    Html::parse(&rsp_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}
