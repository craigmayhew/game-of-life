
use bevy::{
    app::AppExit, // detect app exit events
    prelude::*, //default bevy
};

use crate::AppState;

#[derive(Resource)]
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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let play_button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        // center button
        margin: UiRect::all(Val::Auto),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };
    let quit_button_style = Style {
        width: Val::Px(150.0), 
        height: Val::Px(65.0),
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
        .spawn(ButtonBundle {
            style: play_button_style,
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(MenuButtonAction::Play)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "New Game",
                button_text_style.clone(),
            ));
        })
        .id();

    let button_quit = commands
        .spawn(ButtonBundle {
            style: quit_button_style,
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(MenuButtonAction::Quit)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Quit",
                button_text_style,
            ));
        })
        .id();
    commands.insert_resource(MenuData { button_play, button_quit });
}

pub fn run(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    // handle button colour changes on hover, press
    for (interaction, mut color, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match menu_button_action {
                    MenuButtonAction::Play => {
                        next_state.set(AppState::NewGame);
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
