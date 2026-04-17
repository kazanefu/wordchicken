use crate::debug_log;
use crate::result::{GameResult, ResultList};
use crate::set_target::{TargetText, TargetTextTextUI};
use crate::state_manager::GameState;
use crate::util::words::{EmbeddingModelResource, WordsResource, random_choices};
use bevy::prelude::*;
use words_lib::words::Word;

const MAX_TURN: usize = 10;
const QUIT_ID: usize = 4;
pub const THRESHOLD: f32 = 0.75;

pub struct GuessPlugin;

impl Plugin for GuessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Guess),
            (reset_resources, reset_turn_count, setup_guess_ui).chain(),
        )
        .insert_resource(Options(Vec::new()))
        .insert_resource(TurnCount(0))
        .insert_resource(CurrentGameResult(GameResult::default()))
        .add_message::<Choosed>()
        .add_message::<BreakThreshold>()
        .add_systems(
            Update,
            (
                update_user_ans_ui,
                get_choose,
                update_options_ui,
                push_choosed_word,
                handle_quit,
                handle_break_threshold,
                press_quit,
                update_target_text_ui_on_guessing,
            )
                .run_if(in_state(GameState::Guess)),
        );
    }
}

#[derive(Message)]
struct BreakThreshold;

#[derive(Resource)]
struct TurnCount(usize);

#[derive(Resource)]
struct Options(Vec<Word>);

#[derive(Message)]
struct Choosed(usize); // 0..4 => choosed word's id, else => quit

#[derive(Resource)]
struct UserAns(Word);

#[derive(Component)]
struct UserAnsUI;

#[derive(Component)]
struct OptionButton(usize);

#[derive(Component)]
struct OptionText(usize);

#[derive(Resource)]
struct TargetWord(Word);

#[derive(Component)]
struct QuitButton;

#[derive(Resource)]
struct CurrentGameResult(GameResult);

#[allow(clippy::too_many_arguments)]
fn reset_resources(
    mut commands: Commands,
    model_res: Res<EmbeddingModelResource>,
    user_ans_option: Option<ResMut<UserAns>>,
    target_word_option: Option<ResMut<TargetWord>>,
    mut target_text: ResMut<TargetText>,
    mut options: ResMut<Options>,
    words: Res<WordsResource>,
    mut current_game_result: ResMut<CurrentGameResult>,
) {
    match user_ans_option {
        Some(mut user_ans) => *user_ans = UserAns(Word::new("keywords: ", &model_res.0)),
        None => commands.insert_resource(UserAns(Word::new("keywords: ", &model_res.0))),
    }
    match target_word_option {
        Some(mut target_word) => target_word.0 = Word::new(&target_text.0, &model_res.0),
        None => commands.insert_resource(TargetWord(Word::new(&target_text.0, &model_res.0))),
    }
    current_game_result.0 = GameResult::default();
    current_game_result.0.target_text = std::mem::take(&mut target_text.0);
    options.0 = random_choices(&words.0);
}

fn reset_turn_count(mut turn_count: ResMut<TurnCount>) {
    turn_count.0 = 0;
}

fn setup_guess_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(GameState::Guess),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                TargetTextTextUI,
                Text::new(""),
                TextFont {
                    font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Left),
                TextColor::WHITE,
            ),
            (
                UserAnsUI,
                Text::new(""),
                TextFont {
                    font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Left),
                TextColor::WHITE,
            ),
            (
                OptionButton(0),
                Button,
                Node {
                    width: percent(40),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    OptionText(0),
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            ),
            (
                OptionButton(1),
                Button,
                Node {
                    width: percent(40),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    OptionText(1),
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            ),
            (
                OptionButton(2),
                Button,
                Node {
                    width: percent(40),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    OptionText(2),
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            ),
            (
                OptionButton(3),
                Button,
                Node {
                    width: percent(40),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    OptionText(3),
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            ),
            (
                QuitButton,
                Button,
                Node {
                    width: percent(20),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    Text::new("Quit"),
                    TextFont {
                        font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            ),
        ],
    ));
}

fn update_user_ans_ui(mut query: Query<&mut Text, With<UserAnsUI>>, user_ans: Res<UserAns>) {
    if !user_ans.is_changed() {
        return;
    }
    use std::fmt::Write;
    let mut text_ui = query
        .single_mut()
        .expect("Expected exactly one user_ans_ui but found none or multiple.");
    text_ui.clear();
    write!(**text_ui, "{}", user_ans.0.text).expect("fail to write! to text_ui");
}

fn update_options_ui(mut query: Query<(&OptionText, &mut Text)>, options: Res<Options>) {
    use std::fmt::Write;
    if options.is_changed() {
        return;
    }
    for (option_id, mut text) in query.iter_mut() {
        text.clear();
        write!(
            **text,
            "{}. {}",
            option_id.0,
            options
                .0
                .get(option_id.0)
                .expect("fail to get option.")
                .text
        )
        .expect("fail to write! to options_text ui");
    }
}

type QuitButtonInputs = (Changed<Interaction>, With<QuitButton>);

fn press_quit(
    mut choose_event: MessageWriter<Choosed>,
    mut query: Query<(&Interaction, &mut BackgroundColor), QuitButtonInputs>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                choose_event.write(Choosed(QUIT_ID));
                break;
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.7, 0.7, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}

fn get_choose(
    mut choose_event: MessageWriter<Choosed>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &OptionButton), Changed<Interaction>>,
) {
    for (interaction, mut background_color, button_id) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                choose_event.write(Choosed(button_id.0));
                break;
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.7, 0.7, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn push_choosed_word(
    mut choosed_event: MessageReader<Choosed>,
    mut options: ResMut<Options>,
    mut user_ans: ResMut<UserAns>,
    model_res: Res<EmbeddingModelResource>,
    words_res: Res<WordsResource>,
    target_word: Res<TargetWord>,
    mut break_threshold_event: MessageWriter<BreakThreshold>,
    mut current_game_result: ResMut<CurrentGameResult>,
) {
    for choosed_id in choosed_event.read().map(|x| x.0) {
        if !(0..4).contains(&choosed_id) {
            return;
        }
        let choosed_text = options.0.swap_remove(choosed_id).text;
        user_ans
            .0
            .join_string_with_comma(choosed_text, &model_res.0);
        options.0 = random_choices(&words_res.0);
        let similarity = target_word.0.cosine_similarity(&user_ans.0);
        debug_log!("similarity: {similarity}",);
        current_game_result.0.score_transition.push(similarity);
        if similarity >= THRESHOLD {
            break_threshold_event.write(BreakThreshold);
        }
    }
}

fn handle_quit(
    mut turn_count: ResMut<TurnCount>,
    mut choosed_event: MessageReader<Choosed>,
    mut game_state: ResMut<NextState<GameState>>,
    mut result_list: ResMut<ResultList>,
    mut current_game_result: ResMut<CurrentGameResult>,
) {
    for event in choosed_event.read() {
        turn_count.0 += 1;
        debug_log!("turn: {}", turn_count.0);
        if turn_count.0 >= MAX_TURN || event.0 == QUIT_ID {
            result_list
                .0
                .push(std::mem::take(&mut current_game_result.0));
            game_state.set(GameState::Result);
        }
    }
}

fn handle_break_threshold(
    mut game_state: ResMut<NextState<GameState>>,
    mut break_threshold: MessageReader<BreakThreshold>,
    mut current_game_result: ResMut<CurrentGameResult>,
    mut result_list: ResMut<ResultList>,
) {
    for _ in break_threshold.read() {
        debug_log!("break the threshold");
        result_list
            .0
            .push(std::mem::take(&mut current_game_result.0));
        game_state.set(GameState::Result);
    }
}

fn update_target_text_ui_on_guessing(
    mut query: Query<&mut Text, With<TargetTextTextUI>>,
    target_text: Res<CurrentGameResult>,
) {
    for mut text in query.iter_mut() {
        **text = format!("Target Sentence: {}", target_text.0.target_text);
    }
}
