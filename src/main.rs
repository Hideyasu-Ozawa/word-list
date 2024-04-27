use std::fs::File;
use vibrato::{Dictionary, Tokenizer};

fn main() {
    let home_dir = "dict"; // 辞書ファイルを置いた場所に書き換えて。
                           // let dict_path = "bccwj-suw+unidic-cwj-3_1_1/system.dic.zst";
                           // let dict_path = "bccwj-suw+unidic-cwj-3_1_1-extracted+compact/system.dic.zst";
    let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";
    let dict_full_path = format!("{}/{}", home_dir, dict_path);
    // let user_lex_csv = format!("{}/{}", home_dir, "user_lex2.csv"); // ユーザ辞書

    // 辞書ファイルのロード
    let reader = zstd::Decoder::new(File::open(dict_full_path).unwrap()).unwrap();
    let dict = Dictionary::read(reader).unwrap();
    // let dict = dict.reset_user_lexicon_from_reader(Some(File::open(user_lex_csv).unwrap())).unwrap();

    // トークナイザーの生成
    let tokenizer = Tokenizer::new(dict)
        .ignore_space(true)
        .unwrap()
        .max_grouping_len(24);

    // ワーカーの生成。mutableです。
    let mut worker = tokenizer.new_worker();

    // 形態素解析する文章
    let text = "GitHub is the home for all developers and Universe is an opportunity for our community to explore the future of software development together. We produce Universe for enterprises, startups, open source communities, security officers, partners and any developer interested in discovering how GitHub can help them accelerate their development experience.";

    // 文章をセット。繰り返したい場合は、これを再度呼び出し、ワーカーを使い回す。
    worker.reset_sentence(text);
    worker.tokenize(); // 形態素解析の実行。mutable self

    println!("num_tokens: {}", worker.num_tokens());

    // 抽出したトークンをループで表示する
    worker
        .token_iter()
        .filter(|t| {
            // 絞り込み
            let words: Vec<&str> = t.feature().split(',').collect();
            let subwords: Vec<&str> = words[0].split('-').collect();
            subwords[0] == "名詞" || subwords[0] == "カスタム名詞"
        })
        .for_each(|t| {
            // 出力
            println!("{}: {}", t.surface(), t.feature());
        });
}
