use bevy::{
    prelude::*, //default bevy
};

#[derive(Resource)]
pub struct MenuData {
    paused: Entity,
}
// A unit struct to help identify PAUSED text UI component, since there may be many Text components
#[derive(Component)]
pub struct PausedText;

pub fn enter(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let font = Font::try_from_bytes(crate::FONT_BYTES.into()).unwrap();
    let font_handle = fonts.add(font);
    let text_style = TextStyle {
        font: font_handle,
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    let paused = commands
        .spawn(
            TextBundle::from_section("PAUSED", text_style)
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    ..default()
                }),
        )
        .insert(PausedText)
        .id();
    commands.insert_resource(MenuData { paused });
}

pub fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.paused).despawn_recursive();
}
