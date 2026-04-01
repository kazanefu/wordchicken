use std::error::Error;

use rand::seq::IndexedRandom;
use words_lib::{embedding::embedding_model::EmbeddingModel, save::SavedEmbeddings, words::Word};
#[derive(Clone, Copy)]
enum GameResult {
    Clear(f32),
    Fail,
    GiveUp,
}
#[derive(Debug, Clone, Copy)]
enum OnGameError {
    StdinInputFailed,
    InvalidTargetInputNotASCII,
    InvalidTargetTooShort,
    ParseFaild,
    GetWordsFailed,
    InvalidSelectOver,
}
#[derive(Clone, Copy)]
enum GuessResult {
    Safe(f32),
    Out,
    Quit,
    GiveUp,
}
impl OnGameError {
    fn error_id(self) -> i32 {
        match self {
            Self::StdinInputFailed => 1,
            Self::InvalidTargetInputNotASCII => 3,
            Self::InvalidTargetTooShort => 4,
            Self::ParseFaild => 5,
            Self::GetWordsFailed => 6,
            Self::InvalidSelectOver => 7,
        }
    }
    fn error_msg(self) -> &'static str {
        match self {
            Self::StdinInputFailed => "failed to get input from stdin!",
            Self::InvalidTargetInputNotASCII => "Target text must be ASCII",
            Self::InvalidTargetTooShort => "Target text must be longer than 5 words",
            Self::ParseFaild => "failed to parse input to usize",
            Self::GetWordsFailed => "faied to get word list from included bytes",
            Self::InvalidSelectOver => "Select number must be in the range:1..=4",
        }
    }
}
impl std::fmt::Display for OnGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: ID: {} {:?}, Message: {}",
            self.error_id(),
            self,
            self.error_msg()
        )
    }
}
impl Error for OnGameError {}

static EMBEDDINGS_BYTES: &[u8] = include_bytes!("../../../assets/embedding.bin");
fn main() -> Result<(), OnGameError> {
    let saved_embeddings =
        SavedEmbeddings::from_bytes(EMBEDDINGS_BYTES).map_err(|_| OnGameError::GetWordsFailed)?;
    let words: Vec<Word> = saved_embeddings
        .words
        .into_iter()
        .zip(saved_embeddings.embeddings)
        .map(|(text, embedding)| Word { text, embedding })
        .collect();
    let mut game_result_stack: Vec<GameResult> = Vec::new();
    let model_res = EmbeddingModel::new();
    loop {
        let game_result = on_game(&model_res, &words);

        match game_result {
            Ok(GameResult::Clear(score)) => {
                println!("Score: {}", score);
                game_result_stack.push(game_result.unwrap()); // unwrap: because it is already checked that game_result is Ok
            }
            Ok(GameResult::Fail) => {
                println!("Failed");
                game_result_stack.push(game_result.unwrap()); // unwrap: because it is already checked that game_result is Ok
            }
            Ok(GameResult::GiveUp) => {
                println!("Gave up");
                game_result_stack.push(game_result.unwrap()); // unwrap: because it is already checked that game_result is Ok
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        println!("will you continue?(y/n)");
        let mut yes_no = String::new();
        std::io::stdin()
            .read_line(&mut yes_no)
            .map_err(|_| OnGameError::StdinInputFailed)?;
        if yes_no.trim() == "n" {
            break;
        }
    }
    Ok(())
}
const THRESHOLD: f32 = 0.75;
fn on_game(model_res: &EmbeddingModel, words: &[Word]) -> Result<GameResult, OnGameError> {
    println!("New game started!\nThreshold: {}", THRESHOLD);
    // initialize user_ans with "keywords: "
    // user_ans: "keywords: foo, bar, ..."
    let mut user_ans = Word::new("keywords: ", model_res);

    let mut target_text = String::new();
    println!("Input the target text.");
    std::io::stdin()
        .read_line(&mut target_text)
        .map_err(|_| OnGameError::StdinInputFailed)?;
    target_text = target_text.trim().to_string();
    is_valid_target_input(&target_text)?;
    let target_text = Word::new(&target_text, model_res);

    let max_guess = 10;
    let mut score = 0.0;
    for i in 0..max_guess {
        println!("turn: {}", i);
        let guess_result = guess(model_res, &target_text, &mut user_ans, words)?;
        match guess_result {
            GuessResult::Safe(new_score) => score = new_score,
            GuessResult::GiveUp => return Ok(GameResult::GiveUp),
            GuessResult::Out => return Ok(GameResult::Fail),
            GuessResult::Quit => return Ok(GameResult::Clear(score)),
        }
        println!("debug log: score: {}", score); // for debug
    }
    Ok(GameResult::Clear(score))
}
fn is_valid_target_input(target_input: &str) -> Result<(), OnGameError> {
    if !target_input.is_ascii() {
        return Err(OnGameError::InvalidTargetInputNotASCII);
    }
    if target_input.split_whitespace().count() < 5 {
        return Err(OnGameError::InvalidTargetTooShort);
    }
    Ok(())
}

fn guess(
    model_res: &EmbeddingModel,
    target_text: &Word,
    user_ans: &mut Word,
    words: &[Word],
) -> Result<GuessResult, OnGameError> {
    let mut rng = rand::rng();
    let mut options: Vec<Word> = words.sample(&mut rng, 4).cloned().collect();
    let mut show_options = String::new();
    options
        .iter()
        .map(|x| &x.text)
        .enumerate()
        .for_each(|(index, text)| show_options.push_str(&format!("{}. {}, ", index + 1, text)));
    println!("Choose from: {}", show_options);

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| OnGameError::StdinInputFailed)?;
    if input.trim() == "give up" {
        return Ok(GuessResult::GiveUp);
    }
    if input.trim() == "quit" {
        return Ok(GuessResult::Quit);
    }
    let select_num: usize = input.trim().parse().map_err(|_| OnGameError::ParseFaild)?;
    // select_num must be 1,2,3 or 4.
    if select_num >= 5 {
        return Err(OnGameError::InvalidSelectOver);
    }

    let selected_word = options.swap_remove(select_num - 1);
    user_ans.join_string_with_comma(selected_word.text, model_res);

    let similarity = user_ans.cosine_similarity(target_text);
    if similarity <= THRESHOLD {
        Ok(GuessResult::Safe(similarity))
    } else {
        Ok(GuessResult::Out)
    }
}
