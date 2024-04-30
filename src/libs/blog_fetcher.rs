use reqwest;
use select::{document::Document, predicate::Name};

use crate::libs::constants::BLOG_MAIN_PAGE_URL;

pub async fn fetch_blog_urls() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let main_page_url = BLOG_MAIN_PAGE_URL;
    let response = reqwest::get(main_page_url).await?;

    let mut urls: Vec<String> = Vec::new();
    if response.status().is_success() {
        let body = response.text().await?;
        let document = Document::from(body.as_str());

        for node in document.find(Name("a")) {
            if let Some(href) = node.attr("href") {
                if href.contains("/202") {
                    urls.push(href.to_string());
                }
            }
        }
    } else {
        eprintln!("Failed to fetch the page");
    }
    urls.sort();
    urls.dedup();

    println!("Blog List");
    println!("{}", "#".repeat(150));
    println!("{}", "#".repeat(150));
    for url in &urls {
        println!("{}", url);
    }

    println!("{}", "#".repeat(150));
    println!("{}", "#".repeat(150));

    Ok(urls)

    // let tmp_urls = urls[0..10].to_vec();
    // Ok(tmp_urls)
}
