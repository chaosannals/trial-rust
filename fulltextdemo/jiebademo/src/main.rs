use jieba_rs::{Jieba, TextRank, TfIdf, KeywordExtract};

fn main() {
    let jieba = Jieba::new();
    let words = jieba.cut("我们中出了一个叛徒", false);
    println!("cut: {:?}", words);
    let words = jieba.cut_for_search("我们中出了一个叛徒", true);
    println!("cut_for_search: {:?}", words);
    let text_rank = TextRank::default();
    let text_rank_top_k = text_rank.extract_keywords(
        &jieba,
        "我们中出了一个叛徒",
        6,
        vec![String::from("ns"), String::from("n"), String::from("vn"), String::from("v")],
    );
    println!("text rank: {:?}", text_rank_top_k);

    let tfidf = TfIdf::default();
    let tfidf_top_k = tfidf.extract_keywords(
        &jieba,
        "我们中出了一个叛徒",
        3,
        vec![String::from("ns"), String::from("n"), String::from("vn"), String::from("v")],
    );
    println!("tfidf: {:?}", tfidf_top_k);
}
