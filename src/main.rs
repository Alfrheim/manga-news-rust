extern crate reqwest;
extern crate select;
use select::predicate::Attr;
use select::document::Document;
mod mangapanda;

fn main() {
    println!("Hello, world!");
        let mut response = reqwest::get("https://pepaloves.com/en/home/3194-flowers-stripes-dress.html")
        .expect("Failed to send request");
    println!("{}", response.status());
    let buf = response.text().expect("Failed to read response");
    let document = Document::from(&buf[..]);
    for node in document.find(Attr("id", "our_price_display")) {
        println!("our price display: {} ", node.text());
    }
}
