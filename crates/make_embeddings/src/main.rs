use indicatif::ProgressBar;

use rayon::prelude::*;
use words_lib::{
    csv_to_strings::load_words,
    embedding::{embedding_model::EmbeddingModel, words_to_vec::batch_embedding},
    save::*,
    words::*,
};

static WORDS_CSV: &str = include_str!("words.csv");

fn main() -> anyhow::Result<()> {
    println!("Start making a embedding.bin");
    let words_batch = embedding_with_batch();
    save(words_batch);
    Ok(())
}

fn embedding_with_batch() -> Words {
    let model = EmbeddingModel::default();
    let strings = load_words(WORDS_CSV);
    let pb = ProgressBar::new(strings.len() as u64);
    let batch = 512;
    let words = strings
        .par_chunks(batch)
        .flat_map(|chunk| {
            let embeddings = batch_embedding(
                &chunk.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                &model,
            );
            pb.inc(chunk.len() as u64);
            chunk
                .iter()
                .zip(embeddings)
                .map(|(text, emb)| Word {
                    text: text.clone(),
                    embedding: emb,
                })
                .collect::<Words>()
        })
        .collect::<Words>();
    pb.finish();
    words
}

fn save(words: Words) {
    let saver = SavedEmbeddings::new(
        words.iter().map(|w| w.text.clone()).collect(),
        words.iter().map(|w| w.embedding.clone()).collect(),
    );
    let embedding_path = "assets/embedding.bin";
    saver.save(embedding_path);
    println!("Completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn embedding_each_words() -> Words {
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
        words
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let mut dot = 0.0;
        let mut na = 0.0;
        let mut nb = 0.0;

        for i in 0..a.len() {
            dot += a[i] * b[i];
            na += a[i] * a[i];
            nb += b[i] * b[i];
        }

        dot / (na.sqrt() * nb.sqrt())
    }
    #[test]
    fn test_main() -> anyhow::Result<()> {
        println!("Start making a embedding.bin");
        let words_batch = embedding_with_batch();
        let words_each = embedding_each_words();
        for (w1, w2) in words_batch.iter().zip(words_each.iter()) {
            let sim = cosine_similarity(&w1.embedding, &w2.embedding);
            assert!(sim > 0.999 && sim < 1.001);
        }
        Ok(())
    }
}
