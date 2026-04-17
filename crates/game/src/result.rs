use crate::guess::THRESHOLD;
use crate::state_manager::GameState;
use bevy::prelude::*;

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResultList(Vec::new()))
            .add_systems(OnEnter(GameState::Result), setup_result_ui)
            .add_systems(
                Update,
                update_retry_button.run_if(in_state(GameState::Result)),
            );
    }
}

#[derive(Default)]
pub struct GameResult {
    pub target_text: String,
    pub score_transition: Vec<f32>,
}

#[derive(Resource)]
pub struct ResultList(pub Vec<GameResult>);

#[derive(Component)]
struct RetryButton;

fn setup_result_ui(
    mut commands: Commands,
    result_list: Res<ResultList>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf");
    let default_game_result = GameResult::default();
    let target = &result_list
        .0
        .last()
        .unwrap_or(&default_game_result)
        .target_text;
    let score = *result_list
        .0
        .last()
        .unwrap_or(&default_game_result)
        .score_transition
        .last()
        .unwrap_or(&0.0);
    let is_clear = score < THRESHOLD;
    commands.spawn((
        DespawnOnExit(GameState::Result),
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
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![
                    (
                        Text::new(if !is_clear { "Game Over" } else { "" }),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.0, 0.0)),
                    ),
                    (
                        Text::new(if is_clear { "Clear" } else { "" }),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.0)),
                    ),
                    (
                        Text::new(format!("Target Sentence: {}", target)),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Node::default(),
                        children![
                            (
                                Text::new("Score: "),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ),
                            (
                                Text::new(format!("{}", score)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 1.0, 0.5)),
                            )
                        ]
                    ),
                ]
            ),
            (
                Button,
                RetryButton,
                Node {
                    width: percent(20),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    Text::new("Retry"),
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
type RetryButtonInputs = (Changed<Interaction>, With<RetryButton>);
fn update_retry_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), RetryButtonInputs>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                game_state.set(GameState::Start);
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
