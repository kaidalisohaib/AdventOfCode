use scraper::{Html, Selector};
use std::thread;
use html2text::from_read;
fn main() {
    println!("Hello, world!");

    let body: String = get_html("https://adventofcode.com/events");
    let document: Html = Html::parse_document(&body);
    let selector = Selector::parse("main>article>div.eventlist-event>a").unwrap();
    
    for element in document.select(&selector){
        println!("=======================================================");
        check_year(element.value().attr("href").unwrap());
        thread::sleep_ms(1_000);
        break;
    }

}

fn check_year(sub_link: &str){
    println!("Check year: {}",sub_link);
    let html: String = get_html(format!("https://adventofcode.com{}",sub_link).as_str());
    let document: Html = Html::parse_document(&html);
    let selector: Selector = Selector::parse("main>pre.calendar>a").unwrap();
    for element in document.select(&selector){
        check_day(element.value().attr("href").unwrap());
        thread::sleep_ms(500);
        break;
    }
}

fn check_day(sub_link: &str){
    println!("   Check day: {}",sub_link);
    let html: String = get_html(format!("https://adventofcode.com{}",sub_link).as_str());
    let document: Html = Html::parse_document(&html);
    let selector: Selector = Selector::parse("body>main>article").unwrap();
    println!("{}", from_read(document.select(&selector).next().unwrap().html().as_bytes(),100));
    
}

fn get_html(link: &str) -> String{
    let mut resp = reqwest::blocking::get(link).unwrap();
    assert!(resp.status().is_success());
    let mut content: Vec<u8> = vec![];
    resp.copy_to(&mut content).unwrap();
    let body: String = String::from_utf8_lossy(&content).to_string();
    return body;
}
