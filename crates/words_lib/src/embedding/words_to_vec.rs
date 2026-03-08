use super::embedding_model::EmbeddingModel;
use candle_core::Tensor;


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