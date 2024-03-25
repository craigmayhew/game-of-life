use bevy::{
    prelude::*, //default bevy
};

#[derive(Resource)]
pub struct MenuData {
    credits_left: Entity,
    credits_right: Entity,
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
    let credits_left_str = r#"
    
    Concept:
    Rust Engineer:
    Soundtrack & SFX:       
    "#;

    let credits_right_str = r#"
    CREDITS
    Craig Mayhew
    Craig Mayhew
    Anthony Howell      
    "#;

    let credits_left = commands
        .spawn(
            TextBundle::from_section(credits_left_str, text_style.clone())
                .with_text_justify(JustifyText::Right)
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
                        right: Val::Px(400.0),
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    ..default()
                }),
        )
        .insert(CreditsText)
        .id();
    let credits_right = commands
        .spawn(
            TextBundle::from_section(credits_right_str, text_style.clone())
                .with_text_justify(JustifyText::Left)
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    padding: UiRect {
                        left: Val::Px(400.0),
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    ..default()
                }),
        )
        .insert(CreditsText)
        .id();
    commands.insert_resource(MenuData {
        credits_left,
        credits_right,
    });
}

pub fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.credits_left).despawn_recursive();
    commands.entity(menu_data.credits_right).despawn_recursive();
}
