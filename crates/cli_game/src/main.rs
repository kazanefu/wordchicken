use words_lib::{words::Word,embedding::{embedding_model,words_to_vec}};
#[derive(Clone, Copy)]
enum GameResult {
    Clear(f32),
    Fail,
    End,
}
#[derive(Debug,Clone, Copy)]
enum OnGameError {
    Unknown,
}
impl OnGameError {
    fn error_id(self)->i32{
        match self {
            Self::Unknown => 0
        }
    }
    fn error_msg(self)->String{
        match self {
            Self::Unknown => String::from("Unknown Error happened!")
        }
    }
}
fn main() {
    let mut game_result_stack:Vec<GameResult> = Vec::new();
    let model_res = embedding_model::EmbeddingModel::new();
    loop {
        let game_result = on_game(&model_res);
        match game_result {
            Ok(GameResult::End) => {break;},
            Ok(GameResult::Clear(_)) | Ok(GameResult::Fail) => {game_result_stack.push(game_result.unwrap());}, // unwrap: because it is already checked that game_result is Ok
            Err(e) => {println!("Error: ID: {} {:?}, Message: {}",e.error_id(),e,e.error_msg());continue;},
        }
    }
}

fn on_game(model_res:&embedding_model::EmbeddingModel)->Result<GameResult,OnGameError> {
    let mut user_ans = Word::new("keywords: ", model_res); 
    Ok(GameResult::End)
}
