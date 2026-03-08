use candle_core::Device;
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use std::sync::Arc;
use tokenizers::Tokenizer;

#[derive(Clone)]
pub struct EmbeddingModel {
    pub model: Arc<BertModel>,
    pub tokenizer: Arc<Tokenizer>,
    pub device: Device,
}

impl EmbeddingModel {
    pub fn new() -> Self {
        let device = if candle_core::utils::cuda_is_available() {
            Device::new_cuda(0).unwrap_or(Device::Cpu)
        } else if candle_core::utils::metal_is_available() {
            Device::new_metal(0).unwrap_or(Device::Cpu)
        } else {
            Device::Cpu
        };
        let api = hf_hub::api::sync::Api::new().expect("fail to get api!");
        let repo = api.model("sentence-transformers/all-MiniLM-L6-v2".to_string());

        let config_file = repo.get("config.json").expect("fail to get config.json");
        let tokenizer_file = repo
            .get("tokenizer.json")
            .expect("fail to get tokenizer.json");
        let weights_file = repo
            .get("model.safetensors")
            .expect("fail to get model.safetensors");

        let config: Config = serde_json::from_reader(
            std::fs::File::open(config_file).expect("fail to open config_file"),
        )
        .expect("fail to from_reader");

        let tokenizer = Tokenizer::from_file(tokenizer_file).expect("fail to tokenizer::from_file");

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_file], candle_core::DType::F32, &device)
                .unwrap()
        };
        let model = BertModel::load(vb, &config).unwrap();
        Self {
            model: Arc::new(model),
            tokenizer: Arc::new(tokenizer),
            device,
        }
    }
}

impl Default for EmbeddingModel{
    fn default() -> Self {
        Self::new()
    }
}
