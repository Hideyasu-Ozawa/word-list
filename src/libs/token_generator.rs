use scraper::{Html, Selector};
use vibrato::Tokenizer;

use super::stop_words::create_stop_words;

pub async fn generate_tokens(tokenizer: Tokenizer, urls: Vec<String>) -> Vec<String> {
    let mut all_tokens = Vec::new();

    let mut worker = tokenizer.new_worker();
    for url in urls {
        println!("Fetching: {}", url);
        let text = extract_text(url).await;

        worker.reset_sentence(text);
        worker.tokenize();

        let tokens: Vec<String> = worker
            .token_iter()
            .map(|t| t.surface().to_string())
            .collect();
        println!("Tokens length: {:?}", tokens.len());
        let mut filterd_tokens = filter_tokens(tokens);

        all_tokens.append(&mut filterd_tokens)
    }

    println!("ALL Tokens length: {:?}", all_tokens.len());
    println!("{}", "#".repeat(150));
    println!("{}", "#".repeat(150));
    all_tokens
}

async fn extract_text(url: String) -> String {
    let resp = reqwest::get(url).await.unwrap();
    let body = resp.text().await.unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("body").unwrap();
    let body_element = document.select(&selector).next().unwrap();
    body_element.text().collect::<String>()
}

fn filter_tokens(tokens: Vec<String>) -> Vec<String> {
    let stop_words = create_stop_words();

    let filtered_tokens: Vec<String> = tokens
        .into_iter()
        .filter(|token| !stop_words.contains(token))
        .collect();

    // println!("Filtered tokens: {:?}", filtered_tokens);
    // println!("Filtered tokens length: {:?}", filtered_tokens.len());

    filtered_tokens
}
