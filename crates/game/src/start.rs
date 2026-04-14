use bevy::prelude::*;
use crate::state_manager::GameState;
pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), setup_start_ui);   
    }
}

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
        children![(
            Text::new("説明"),
            TextFont {
                font: asset_server.load("embedded://game/fonts/NotoSansJP-Bold.ttf"),
                font_size: 40.0,
                ..default()
            },
            TextLayout::new_with_justify(Justify::Left),
            TextColor::WHITE,

        ),(
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
        )]
    )
}

fn setup_start_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(start_ui_bundle(asset_server));
} 