use serde::{Deserialize,Serialize};
// ==========================
// Save / Load
// ==========================

#[derive(Serialize, Deserialize)]
pub struct SavedEmbeddings {
    pub words: Vec<String>,
    pub embeddings: Vec<Vec<f32>>,
}


impl SavedEmbeddings{
    pub fn new(words:Vec<String>,embeddings: Vec<Vec<f32>>)-> Self{
        Self { words, embeddings }
    }
    pub fn load(path: &str)->Result<Self,std::io::Error>{
        let bytes = std::fs::read(path)?;
        Ok(bincode::deserialize(&bytes).expect("fail to deserialize"))
    }
    pub fn save(&self,path: &str){
        save_embeddings(path, self);
    }
}

fn save_embeddings(path: &str, data: &SavedEmbeddings) {
    let encoded = bincode::serialize(data).unwrap();
    std::fs::write(path, encoded).unwrap();
}