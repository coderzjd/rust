fn main() {
   trpl::run(async {
        let body = reqwest::get("https://www.rust-lang.org")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("body = {body:?}");
    })
}
