use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use csv::ReaderBuilder;
use directories::ProjectDirs;

use crate::{EMBEDDINGS_BYTES, WORDS_CSV};
use words_lib::{embedding::embedding_model::EmbeddingModel, save::SavedEmbeddings, words::{Word,Words}};

// ==========================
// Resources
// ==========================

#[derive(Resource, Clone)]
pub struct EmbeddingModelResource(pub EmbeddingModel);

#[derive(Resource)]
pub struct WordsResource(pub Words);

#[derive(Resource)]
struct EmbeddingTask(Task<SavedEmbeddings>);

// ==========================
// Plugin
// ==========================

pub struct WordPlugin;

impl Plugin for WordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_model)
            .add_systems(Startup, start_embedding_task.after(setup_model))
            .add_systems(Update, poll_embedding_task);
    }
}

// ==========================
// Model Setup
// ==========================

fn setup_model(mut commands: Commands) {
    commands.insert_resource(EmbeddingModelResource(EmbeddingModel::default()));
}

// ==========================
// Async Embedding Start
// ==========================

fn start_embedding_task(mut commands: Commands, model_res: Res<EmbeddingModelResource>) {
    // キャッシュがあればロード
    if let Ok(saved) = SavedEmbeddings::from_bytes(EMBEDDINGS_BYTES) {
        println!("Loaded included embeddings");

        let words = saved
            .words
            .into_iter()
            .zip(saved.embeddings)
            .map(|(text, embedding)| Word { text, embedding })
            .collect();

        commands.insert_resource(WordsResource(words));
        return;
    }
    let appdata_path = ProjectDirs::from("com", "kazanefu", "wordchicken")
                .expect("fail to get project directory");
    let cache_path = appdata_path.data_dir().join("assets/embedding.bin");
    if let Ok(saved) = SavedEmbeddings::load(cache_path){
        println!("Loaded cached embeddings");

        let words = saved
            .words
            .into_iter()
            .zip(saved.embeddings)
            .map(|(text, embedding)| Word { text, embedding })
            .collect();

        commands.insert_resource(WordsResource(words));
        return;
    }

    println!("No cache or include found. Computing embeddings...");

    let words = load_words();
    let model = model_res.clone().0;

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

fn poll_embedding_task(mut commands: Commands, task_res: Option<ResMut<EmbeddingTask>>) {
    if let Some(mut task) = task_res {
        use futures_lite::future;

        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            println!("Embedding finished!");

            let appdata_path = ProjectDirs::from("com", "kazanefu", "wordchicken")
                .expect("fail to get project directory");

            result.save(appdata_path.data_dir().join("assets/embedding.bin"));

            let words = result
                .words
                .into_iter()
                .zip(result.embeddings)
                .map(|(text, embedding)| Word { text, embedding })
                .collect();

            commands.insert_resource(WordsResource(words));
            commands.remove_resource::<EmbeddingTask>();
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

    rdr.records().map(|r| r.unwrap()[0].to_string()).collect()
}

use rand::prelude::*;

pub fn random_choices(words: &[Word]) -> Vec<Word> {
    let mut rng = rand::rng();
    words.sample(&mut rng, 4).cloned().collect()
}

