use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use csv::ReaderBuilder;
use tokenizers::Tokenizer;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::path::Path;

use crate::WORDS_CSV;

// ==========================
// Resources
// ==========================

#[derive(Resource, Clone)]
pub struct EmbeddingModel {
    pub model: Arc<BertModel>,
    pub tokenizer: Arc<Tokenizer>,
    pub device: Device,
}

#[derive(Resource)]
pub struct Words {
    pub words: Vec<Word>,
}

#[derive(Resource)]
struct EmbeddingTask(Task<SavedEmbeddings>);

// ==========================
// Plugin
// ==========================

pub struct WordPlugin;

impl Plugin for WordPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_model)
            .add_systems(Startup, start_embedding_task.after(setup_model))
            .add_systems(Update, poll_embedding_task);
    }
}

// ==========================
// Model Setup
// ==========================

fn setup_model(mut commands: Commands) {
    let device = if candle_core::utils::cuda_is_available() {
        Device::new_cuda(0).unwrap_or(Device::Cpu)
    } else if candle_core::utils::metal_is_available() {
        Device::new_metal(0).unwrap_or(Device::Cpu)
    } else {
        Device::Cpu
    };

    let api = hf_hub::api::sync::Api::new().unwrap();
    let repo = api.model("sentence-transformers/all-MiniLM-L6-v2".to_string());

    let config_file = repo.get("config.json").unwrap();
    let tokenizer_file = repo.get("tokenizer.json").unwrap();
    let weights_file = repo.get("model.safetensors").unwrap();

    let config: Config =
        serde_json::from_reader(std::fs::File::open(config_file).unwrap()).unwrap();

    let tokenizer = Tokenizer::from_file(tokenizer_file).unwrap();

    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(
            &[weights_file],
            candle_core::DType::F32,
            &device,
        )
        .unwrap()
    };

    let model = BertModel::load(vb, &config).unwrap();

    commands.insert_resource(EmbeddingModel {
        model: Arc::new(model),
        tokenizer: Arc::new(tokenizer),
        device,
    });
}

// ==========================
// Async Embedding Start
// ==========================

fn start_embedding_task(
    mut commands: Commands,
    model_res: Res<EmbeddingModel>,
) {
    // キャッシュがあればロード
    if let Some(saved) = load_embeddings("embeddings.bin") {
        println!("Loaded cached embeddings");

        let words = saved.words
            .into_iter()
            .zip(saved.embeddings)
            .map(|(text, embedding)| Word { text, embedding })
            .collect();

        commands.insert_resource(Words { words });
        return;
    }

    println!("No cache found. Computing embeddings...");

    let words = load_words();
    let model = model_res.clone();

    let task_pool = AsyncComputeTaskPool::get();

    let task = task_pool.spawn(async move {
        let words_struct: Vec<Word> = words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                println!("Embedding {}/{}", i + 1, words.len());
                Word::new(w, &model)
            })
            .collect();

        SavedEmbeddings {
            words: words_struct.iter().map(|w| w.text.clone()).collect(),
            embeddings: words_struct.iter().map(|w| w.embedding.clone()).collect(),
        }
    });

    commands.insert_resource(EmbeddingTask(task));
}

// ==========================
// Poll Async Task
// ==========================

fn poll_embedding_task(
    mut commands: Commands,
    mut task_res: Option<ResMut<EmbeddingTask>>,
){
    if let Some(mut task) = task_res {
        use futures_lite::future;

        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            println!("Embedding finished!");

            let appdata_path = ProjectDirs::from("com","kazanefu","wordchicken").expect("fail to get project directory");



            save_embeddings("embeddings.bin", &result);

            let words = result.words
                .into_iter()
                .zip(result.embeddings)
                .map(|(text, embedding)| Word { text, embedding })
                .collect();

            commands.insert_resource(Words { words });
            commands.remove_resource::<EmbeddingTask>();
        }
    }
}

// ==========================
// Embedding Logic
// ==========================

pub fn str_to_embedding(text: &str, model_res: &EmbeddingModel) -> Vec<f32> {
    let tokenizer = &model_res.tokenizer;
    let model = &model_res.model;
    let device = &model_res.device;

    let tokens = tokenizer.encode(text, true).unwrap();

    let token_ids = Tensor::new(tokens.get_ids(), device).unwrap().unsqueeze(0).unwrap();
    let token_type_ids = Tensor::new(tokens.get_type_ids(), device).unwrap().unsqueeze(0).unwrap();
    let attention_mask = Tensor::new(tokens.get_attention_mask(), device).unwrap().unsqueeze(0).unwrap();

    let embeddings = model
        .forward(&token_ids, &token_type_ids, Some(&attention_mask))
        .unwrap();

    let (_b, n_tokens, _h) = embeddings.dims3().unwrap();
    let mean = (embeddings.sum(1).unwrap() / (n_tokens as f64)).unwrap();

    let norm = mean.sqr().unwrap().sum_all().unwrap().sqrt().unwrap();
    let normalized = mean.broadcast_div(&norm).unwrap();

    normalized.to_vec2::<f32>().unwrap()[0].clone()
}

// ==========================
// Word Struct
// ==========================

pub struct Word {
    pub text: String,
    pub embedding: Vec<f32>,
}

impl Word {
    pub fn new(text: &str, model_res: &EmbeddingModel) -> Self {
        let embedding = str_to_embedding(text, model_res);
        Self {
            text: text.to_string(),
            embedding,
        }
    }
}

// ==========================
// CSV Load
// ==========================

fn load_words() -> Vec<String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(WORDS_CSV.as_bytes());

    rdr.records()
        .map(|r| r.unwrap()[0].to_string())
        .collect()
}

// ==========================
// Save / Load
// ==========================

#[derive(Serialize, Deserialize)]
pub struct SavedEmbeddings {
    pub words: Vec<String>,
    pub embeddings: Vec<Vec<f32>>,
}

fn save_embeddings(path: &str, data: &SavedEmbeddings) {
    let encoded = bincode::serialize(data).unwrap();
    std::fs::write(path, encoded).unwrap();
}

fn load_embeddings(path: &str) -> Option<SavedEmbeddings> {
    if !Path::new(path).exists() {
        return None;
    }
    let bytes = std::fs::read(path).unwrap();
    Some(bincode::deserialize(&bytes).unwrap())
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
