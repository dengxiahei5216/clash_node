use reqwest::blocking::Client;


fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;
    let res = client.get(url).send()?;
    let body = res.text()?;
    Ok(body)
}


fn main() {
    let url = "https://www.baidu.com";
    let html = get_html(url).unwrap();
    println!("{}", html);
}