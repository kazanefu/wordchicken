use crate::*;
use bevy::prelude::*;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use csv::ReaderBuilder;
use tokenizers::Tokenizer;

#[derive(Resource)]
pub struct EmbeddingModel {
    pub model: BertModel,
    pub tokenizer: Tokenizer,
    pub device: Device,
}

#[derive(Resource)]
pub struct Words {
    pub words: Vec<Word>,
}

pub struct WordPlugin;

impl Plugin for WordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_model, setup_words).chain());
    }
}

fn setup_model(mut commands: Commands) {
    let device = if candle_core::utils::cuda_is_available() {
        Device::new_cuda(0).unwrap_or(Device::Cpu)
    } else if candle_core::utils::metal_is_available() {
        Device::new_metal(0).unwrap_or(Device::Cpu)
    } else {
        Device::Cpu
    };

    // Hugging Face からファイルをアーカイブ
    let api = hf_hub::api::sync::Api::new().unwrap();
    let repo = api.model("sentence-transformers/all-MiniLM-L6-v2".to_string());

    let config_file = repo.get("config.json").unwrap();
    let tokenizer_file = repo.get("tokenizer.json").unwrap();
    let weights_file = repo.get("model.safetensors").unwrap();

    let config: Config =
        serde_json::from_reader(std::fs::File::open(config_file).unwrap()).unwrap();
    let tokenizer = Tokenizer::from_file(tokenizer_file).unwrap();
    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[weights_file], candle_core::DType::F32, &device)
            .unwrap()
    };
    let model = BertModel::load(vb, &config).unwrap();

    commands.insert_resource(EmbeddingModel {
        model,
        tokenizer,
        device,
    });
}

fn setup_words(mut commands: Commands, model_res: Res<EmbeddingModel>) {
    let words_str = load_words();
    let words = words_str
        .iter()
        .map(|word| Word::new(word, &model_res))
        .collect();
    commands.insert_resource(Words { words });
}

pub fn str_to_embedding(text: &str, model_res: &EmbeddingModel) -> Vec<f32> {
    let tokenizer = &model_res.tokenizer;
    let model = &model_res.model;
    let device = &model_res.device;

    // 1. トークナイズ
    let tokens = tokenizer.encode(text, true).unwrap();
    let token_ids = Tensor::new(tokens.get_ids(), device)
        .unwrap()
        .unsqueeze(0)
        .unwrap();
    let token_type_ids = Tensor::new(tokens.get_type_ids(), device)
        .unwrap()
        .unsqueeze(0)
        .unwrap();
    let attention_mask = Tensor::new(tokens.get_attention_mask(), device)
        .unwrap()
        .unsqueeze(0)
        .unwrap();

    // 2. 推論
    let embeddings = model
        .forward(&token_ids, &token_type_ids, Some(&attention_mask))
        .unwrap();

    // 3. Mean Pooling (全トークンの埋め込みの平均を計算)
    // embeddings: [batch, seq_len, hidden_size] -> [hidden_size]
    let (_n_batch, n_tokens, _hidden_size) = embeddings.dims3().unwrap();
    let mean_embedding = (embeddings.sum(1).unwrap() / (n_tokens as f64)).unwrap();

    // 4. L2正規化 (コサイン類似度計算を単純化するため)
    let norm = mean_embedding
        .sqr()
        .unwrap()
        .sum_all()
        .unwrap()
        .sqrt()
        .unwrap();
    let normalized = mean_embedding.broadcast_div(&norm).unwrap();

    normalized.to_vec2::<f32>().unwrap()[0].clone()
}

pub struct Word {
    text: String,
    embedding: Vec<f32>,
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
