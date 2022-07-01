use std::fs;

fn main() {

    println!("Hello, world!");
    let url = "https://www.baidu.com/";
    let output = "rust.md";
    println!("fetching url: {}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("converting html to markdown....");

    let md = html2md::parse_html(&body);

    fs::write(output,md.as_bytes()).unwrap();

    println!("converted markdown has been sabed in {}.", output);

}
