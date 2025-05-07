use trpl::Html;

fn main() {
    trpl::run(async {
        let title = page_title("https://www.baidu_dasdhasd.com/").await;
        match title {
            Some(info) => println!("info is {info}"),
            None => println!("no title"),
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
