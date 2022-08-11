use bevy::{
    prelude::*, //default bevy
};

use crate::{
    SessionResource,
};

pub struct HudData {
    life_counter: Entity,
}

// A unit struct to help identify the life counter UI component, since there may be many Text components
#[derive(Component)]
pub struct LifeCounterText;

pub fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_text_style = TextStyle {
        font: asset_server.load("font/square.ttf"),
        font_size: 20.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    let life_counter = commands
        .spawn_bundle(
            TextBundle::from_section(
                "Life detected: 0000000",
                button_text_style,
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
        )
        .insert(LifeCounterText)
        .id();
    commands.insert_resource(HudData { life_counter });
}

pub fn run(
    session: ResMut<SessionResource>,
    mut query: Query<&mut Text, With<LifeCounterText>>
) {
    for mut text in &mut query {
        text.sections[0].value = format!("Life detected: {:07}",&session.counter);
    }
}

pub fn cleanup(mut commands: Commands, hud_data: Res<HudData>) {
    commands.entity(hud_data.life_counter).despawn_recursive();
}
