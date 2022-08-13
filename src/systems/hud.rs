use bevy::{
    prelude::*, //default bevy
};

use crate::{
    SessionResource,
};

pub struct HudData {
    counters: Entity,
}

// A unit struct to help identify the life counter UI component, since there may be many Text components
#[derive(Component)]
pub struct LifeCounterText;

pub fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("font/square.ttf"),
        font_size: 20.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    let counters = commands
        .spawn_bundle(
            TextBundle::from_section(
                "Life detected: 0000000",
                text_style,
            )
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(LifeCounterText)
        .id();
    commands.insert_resource(HudData { counters });
}

pub fn run(
    session: ResMut<SessionResource>,
    mut life_counter: Query<&mut Text, With<LifeCounterText>>,
) {
    for mut text in &mut life_counter {
        text.sections[0].value = format!("Generation: {:07}\nLife detected: {:07}",&session.generation,&session.counter);
    }
}

pub fn cleanup(mut commands: Commands, hud_data: Res<HudData>) {
    commands.entity(hud_data.counters).despawn_recursive();
}
