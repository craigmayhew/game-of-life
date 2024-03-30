use bevy::{
    prelude::*, //default bevy
};

#[derive(Resource)]
pub struct MenuData {
    credits: Entity,
}
// A unit struct to help identify CREDIT text UI component
#[derive(Component)]
pub struct CreditsText;

pub fn enter(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let font = Font::try_from_bytes(crate::FONT_BYTES.into()).unwrap();
    let font_handle = fonts.add(font);
    let text_style = TextStyle {
        font: font_handle,
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    let credits_left_str = "Concept & Rust Dev\nCraig Mayhew\n\nSoundtrack & SFX\nAnthony Howell";

    let credits = commands
        .spawn(
            TextBundle::from_section(credits_left_str, text_style.clone())
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    padding: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    ..default()
                }),
        )
        .insert(CreditsText)
        .id();
    // add a rectangular border to the credits text
    commands
        .entity(credits)
        .insert(Outline::new(Val::Px(2.), Val::Px(10.0), Color::WHITE));
    commands.insert_resource(MenuData { credits });
}

pub fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.credits).despawn_recursive();
}
