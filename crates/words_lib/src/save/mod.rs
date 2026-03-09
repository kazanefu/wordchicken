use serde::{Deserialize,Serialize};
use std::path::Path;
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
    pub fn load(path: impl AsRef<Path>)->Result<Self,std::io::Error>{
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        Ok(bincode::deserialize(&bytes).expect("fail to deserialize"))
    }
    pub fn save(&self,path: impl AsRef<Path>){
        let path = path.as_ref();
        save_embeddings(path, self);
    }
    pub fn from_bytes(bytes:&[u8])->Result<Self,bincode::Error>{
        bincode::deserialize(bytes)
    }
}

fn save_embeddings(path: &Path, data: &SavedEmbeddings) {
    let encoded = bincode::serialize(data).unwrap();
    std::fs::write(path, encoded).unwrap();
}