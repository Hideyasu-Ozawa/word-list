mod libs;

use libs::blog_fetcher::fetch_blog_urls;
use libs::constants::{MAX_RANKING, TOKENIZER_MAX_GROUPING_LENGTH};
use libs::dictinary_loader::load_dictionary;
use libs::token_generator::generate_tokens;
use libs::token_ranker::rank_tokens;
use reqwest::Error;
use vibrato::Tokenizer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = fetch_blog_urls().await.unwrap();
    println!("blog lentgh: {}", &urls.len());
    let dict = load_dictionary();

    let tokenizer = Tokenizer::new(dict)
        .ignore_space(true)
        .unwrap()
        .max_grouping_len(TOKENIZER_MAX_GROUPING_LENGTH);

    let tokens = generate_tokens(tokenizer, urls).await;

    let ranking = rank_tokens(tokens);

    println!("Ranking");
    println!("{}", "#".repeat(150));
    println!("{}", "#".repeat(150));
    for (i, (token, count)) in ranking.iter().take(MAX_RANKING).enumerate() {
        println!("{:>3}: {} ({} times)", i + 1, token, count);
    }
    println!("{}", "#".repeat(150));
    println!("{}", "#".repeat(150));

    Ok(())
}
