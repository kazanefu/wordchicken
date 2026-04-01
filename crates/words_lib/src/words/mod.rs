use crate::embedding::{embedding_model::EmbeddingModel,words_to_vec::str_to_embedding};
#[derive(Clone)]
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
    pub fn cosine_similarity(&self,rhs:Self)->f32 {
        cosine_similarity(&self.embedding, &rhs.embedding)
    }
    pub fn  push_str(&mut self,addition:&str,model_res: &EmbeddingModel){
        self.text.push_str(addition);
        self.embedding = str_to_embedding(&self.text, model_res);
    }
    pub fn join_string_with_comma(&mut self,mut addition:String,model_res: &EmbeddingModel){
        addition.push_str(", ");
        self.push_str(&addition, model_res);
    }
}
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}
pub type Words = Vec<Word>;