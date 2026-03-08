use indicatif::ProgressBar;

use words_lib::{
    csv_to_strings::load_words, embedding::embedding_model::EmbeddingModel, save::*, words::*,
};
use rayon::prelude::*;

static WORDS_CSV: &str = include_str!("words.csv");

fn main() -> anyhow::Result<()> {
    println!("start make embedding.bin");
    let model = EmbeddingModel::default();
    let strings = load_words(WORDS_CSV);
    let pb = ProgressBar::new(strings.len() as u64);
    let words = strings
        .par_iter()
        .map(|s| {
            let word = Word::new(s, &model);
            pb.inc(1);
            word
        })
        .collect::<Words>();
    pb.finish();
    let saver = SavedEmbeddings::new(
        words.iter().map(|w| w.text.clone()).collect(),
        words.iter().map(|w| w.embedding.clone()).collect(),
    );
    let embedding_path = "assets/embedding.bin";
    saver.save(embedding_path);
    println!("Completed");
    Ok(())
}
