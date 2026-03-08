use crate::embedding::{embedding_model::EmbeddingModel,words_to_vec::str_to_embedding};
pub struct Word {
    pub text: String,
    pub embedding: Vec<f32>
}

impl Word {
    pub fn new(text: &str, model: &EmbeddingModel)-> Self {
        let embedding = str_to_embedding(text, model);
        Self {
            text: text.to_string(),
            embedding
        }
    }
}

pub type Words = Vec<Word>;