use crate::set_target::{TargetText, TargetTextTextUI};
use crate::state_manager::GameState;
use crate::util::words::EmbeddingModelResource;
use bevy::prelude::*;
use words_lib::words::Word;

pub struct GuessPlugin;

impl Plugin for GuessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Guess), setup_guess);
    }
}

#[derive(Resource)]
struct UserAns(Word);

#[derive(Component)]
struct OptionButton(usize);

#[derive(Component)]
struct OptionText(usize);

#[derive(Component)]
struct QuitButton;

fn setup_guess(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    model_res: Res<EmbeddingModelResource>,
    user_ans_option: Option<ResMut<UserAns>>,
) {

    match user_ans_option {
        Some(mut user_ans) => *user_ans = UserAns(Word::new("keywords: ", &model_res.0)),
        None => commands.insert_resource(UserAns(Word::new("keywords: ", &model_res.0))),

    }

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
                OptionButton(0),
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
                    width: percent(20),
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
                    width: percent(20),
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
                    width: percent(20),
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
