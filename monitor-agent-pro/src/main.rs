use std::fs;

// main 函数返回一个 Result
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("fetching url: {}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    println!("converting html to markdown....");

    let md = html2md::parse_html(&body);

    fs::write(output,md.as_bytes())?;

    println!("converted markdown has been sabed in {}.", output);

    Ok(())

}
