// use reqwest::blocking::RequestBuilder;
use html2text::from_read;
use scraper::{Html, Selector};
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let body: String = get_html("https://adventofcode.com/events");
    let document: Html = Html::parse_document(&body);
    let selector = Selector::parse("main>article>div.eventlist-event>a").unwrap();

    for (index, element) in document.select(&selector).enumerate() {
        if (index == 6) {
            println!("=======================================================");
            check_year(element.value().attr("href").unwrap());
            thread::sleep(time::Duration::from_millis(1000));
        }
        // break;
    }
}

fn check_year(sub_link: &str) {
    println!("Check year: {}", sub_link);
    let html: String = get_html(format!("https://adventofcode.com{}", sub_link).as_str());
    let document: Html = Html::parse_document(&html);
    let selector: Selector = Selector::parse("main>pre.calendar>a").unwrap();
    for element in document.select(&selector) {
        check_day(element.value().attr("href").unwrap());
        thread::sleep(time::Duration::from_millis(500));
        // break;
    }
}

fn check_day(sub_link: &str) {
    println!("   Check day: {}", sub_link);
    let html: String = get_html(format!("https://adventofcode.com{}", sub_link).as_str());
    let document: Html = Html::parse_document(&html);
    let selector: Selector = Selector::parse("body>main>article").unwrap();
    for (index, parts) in document.select(&selector).enumerate() {
        println!("      Part: {:?}", index + 1);
    }
    println!("{}", from_read(document.select(&selector).next().unwrap().html().as_bytes(),100));
}

fn get_html(link: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let mut resp = client
        .get(link)
        .basic_auth("asdsa", Some("asdasd"))
        .send()
        .unwrap();

    // let mut resp = reqwest::blocking::get(link).unwrap();
    assert!(resp.status().is_success());
    let mut content: Vec<u8> = vec![];
    resp.copy_to(&mut content).unwrap();
    let body: String = String::from_utf8_lossy(&content).to_string();
    return body;
}
