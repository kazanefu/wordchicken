use crate::state_manager::GameState;
use bevy::prelude::*;
pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), setup_start_ui)
            .add_systems(
                Update,
                press_start_button.run_if(in_state(GameState::Start)),
            );
    }
}

const EXPLANATION: &str = r#"
概要:
英語の意味ベクトルを使ったチキンレースです。目標とする文を最初に入力し、その文の意味ベクトルに近づけるように単語を選択肢から選んで行きます。
制約:
目標とする文章はASCII文字のみかつ、5単語以上
単語は最大10回4択で選びます。途中で終了してその時点でのスコアを最終スコアとすることもできます。
閾値はコサイン類似度0.75です。閾値を超えたらその時点でゲームオーバーです。
コサイン類似度は目標文と"keywords: A, B, C, ..."とつないだ形の文から算出します。
"#;

#[derive(Component)]
struct StartButton;

fn start_ui_bundle(asset_server: Res<AssetServer>) -> impl Bundle {
    (
        DespawnOnExit(GameState::Start),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new(EXPLANATION),
                TextFont {
                    font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Left),
                TextColor::WHITE,
            ),
            (
                Button,
                StartButton,
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
                    Text::new("Start"),
                    TextFont {
                        font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            )
        ],
    )
}

fn setup_start_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(start_ui_bundle(asset_server));
}

type StartButtonInputs = (Changed<Interaction>, With<StartButton>);

fn press_start_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), StartButtonInputs>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                game_state.set(GameState::SetTargetText);
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
