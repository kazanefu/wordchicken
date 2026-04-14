use crate::state_manager::GameState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
pub struct SetTargetTextPlugin;

impl Plugin for SetTargetTextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargetText::default())
            .add_systems(OnEnter(GameState::SetTargetText), setup_set_target_ui)
            .add_systems(
                Update,
                (
                    receive_text.run_if(in_state(GameState::SetTargetText)),
                    update_target_text_ui.run_if(not(in_state(GameState::Start))),
                ),
            );
    }
}

#[derive(Component)]
pub struct TargetTextTextUI;

fn setup_set_target_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut target_text: ResMut<TargetText>) {
    target_text.0.clear();
    commands.spawn((
        DespawnOnExit(GameState::SetTargetText),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![(
            TargetTextTextUI,
            Text::new(""),
            TextFont {
                font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                font_size: 40.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Left),
            TextColor::WHITE,
        )],
    ));
}

#[derive(Resource, Default)]
pub struct TargetText(String);

impl TargetText {
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string);
    }
    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }
}

fn is_valid_target_input(target_input: &str) -> bool {
    target_input.is_ascii() && target_input.split_whitespace().count() >= 5
}

fn receive_text(
    mut events: MessageReader<KeyboardInput>,
    mut target_text: ResMut<TargetText>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }
        use bevy::input::keyboard::Key;

        match &event.logical_key {
            Key::Backspace => {
                target_text.pop();
            }
            Key::Enter => {
                println!("decide: {}", target_text.0);
                if is_valid_target_input(&target_text.0) {
                    game_state.set(GameState::Guess);
                } else {
                    println!("target text:invalid input");
                }
            }
            _ => {
                if let Some(text) = &event.text {
                    target_text.push_str(text);
                }
            }
        }
    }
}

fn update_target_text_ui(
    mut query: Query<&mut Text, With<TargetTextTextUI>>,
    target_text: Res<TargetText>,
) {
    for mut text in query.iter_mut() {
        **text = format!("Target Text: {}", target_text.0);
    }
}
