use crate::*;
use csv::ReaderBuilder;

pub struct Word {
    text: String,
    embedding: Vec<f32>,
}

impl Word {
    pub fn new(text: &str) -> Self {
        let embedding = str_to_embedding(text);
        Self {
            text: text.to_string(),
            embedding,
        }
    }
}

pub fn str_to_embedding(text: &str) -> Vec<f32> {
    todo!()
}

fn load_words() -> Vec<String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(WORDS_CSV.as_bytes());

    rdr.records().map(|r| r.unwrap()[0].to_string()).collect()
}
use rand::prelude::*;

fn random_choices(words: &[String]) -> Vec<String> {
    let mut rng = rand::rng();
    words.sample(&mut rng, 4).cloned().collect()
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}
