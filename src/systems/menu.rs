
use bevy::{
    {app::AppExit}, // detect app exit events
    prelude::*, //default bevy
};

use crate::AppState;

pub struct MenuData {
    button_play: Entity,
    button_quit: Entity,
}

// All actions that can be triggered from a menu button click
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.4, 0.4, 0.4);
pub const HOVERED_BUTTON: Color = Color::rgb(0.6, 0.6, 0.6);
pub const PRESSED_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
        // center button
        margin: UiRect::all(Val::Auto),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("font/square.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    let button_play = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(MenuButtonAction::Play)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Play",
                button_text_style.clone(),
            ));
        })
        .id();

    let button_quit = commands
        .spawn_bundle(ButtonBundle {
            style: button_style,
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(MenuButtonAction::Quit)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Quit",
                button_text_style,
            ));
        })
        .id();
    commands.insert_resource(MenuData { button_play, button_quit });
}

pub fn menu(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    // handle button colour changes on hover, press
    for (interaction, mut color, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match menu_button_action {
                    MenuButtonAction::Play => {
                        state.set(AppState::InGame).unwrap();
                    },
                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_play).despawn_recursive();
    commands.entity(menu_data.button_quit).despawn_recursive();
}
