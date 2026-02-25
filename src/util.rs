use csv::ReaderBuilder;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::spawn_local;
use crate::*;

fn load_words() -> Vec<String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(WORDS_CSV.as_bytes());

    rdr.records()
        .map(|r| r.unwrap()[0].to_string())
        .collect()
}

use rand::prelude::*;

fn random_choices(words: &[String]) -> Vec<String> {
    let mut rng = rand::thread_rng();
    words.choose_multiple(&mut rng, 4)
        .cloned()
        .collect()
}

async fn get_embedding(text: String) -> Vec<f32> {
    let promise = embed(text);
    let js_value = JsFuture::from(promise)
        .await
        .expect("Promise failed");

    let array = js_sys::Float32Array::new(&js_value);
    array.to_vec()
}


fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b).map(|(x,y)| x*y).sum::<f32>();
    let norm_a = a.iter().map(|x| x*x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x*x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}