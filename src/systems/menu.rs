use bevy::{
    app::AppExit, // detect app exit events
    prelude::*,   //default bevy
};

use crate::AppState;

macro_rules! button {
    ($btn_style:expr, $txt_style:expr, $commands:expr, $action:expr, $text:expr) => {
        $commands
            .spawn(ButtonBundle {
                style: $btn_style,
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert($action)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section($text, $txt_style.clone()));
            })
            .id()
    };
}

/// Menu entities (buttons)
#[derive(Resource)]
pub struct MenuEntities {
    button_play: Entity,
    button_load: Entity,
    button_save: Entity,
    button_cred: Entity,
    button_quit: Entity,
}

/// All actions that can be triggered from a menu button click
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Load,
    Save,
    Cred,
    Quit,
}

/// Normal button colour
pub const NORMAL_BUTTON: Color = Color::rgb(0.4, 0.4, 0.4);
/// Hover button colour
pub const HOVERED_BUTTON: Color = Color::rgb(0.6, 0.6, 0.6);
/// Press button colour
pub const PRESSED_BUTTON: Color = Color::rgb(0.9, 0.9, 0.9);

/// Called when entering MENU state
pub fn setup(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let button_style = Style {
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
    let mut button_play_style = button_style.clone();
    let mut button_load_style = button_style.clone();
    let mut button_save_style = button_style.clone();
    let mut button_cred_style = button_style.clone();
    let mut button_quit_style = button_style.clone();
    button_play_style.top = Val::Px(-180.0);
    button_load_style.top = Val::Px(-100.0);
    button_save_style.top = Val::Px(-20.0);
    button_cred_style.top = Val::Px(60.0);
    button_quit_style.top = Val::Px(140.0);

    let font = Font::try_from_bytes(crate::FONT_BYTES.into()).unwrap();
    let font_handle = fonts.add(font);

    let button_text_style = TextStyle {
        font: font_handle,
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    let button_play = button!(
        button_play_style,
        button_text_style,
        commands,
        MenuButtonAction::Play,
        "New Game"
    );
    let button_load = button!(
        button_load_style,
        button_text_style,
        commands,
        MenuButtonAction::Load,
        "Load Game"
    );
    let button_save = button!(
        button_save_style,
        button_text_style,
        commands,
        MenuButtonAction::Save,
        "Save Game"
    );
    let button_cred = button!(
        button_cred_style,
        button_text_style,
        commands,
        MenuButtonAction::Cred,
        "Credits"
    );
    let button_quit = button!(
        button_quit_style,
        button_text_style,
        commands,
        MenuButtonAction::Quit,
        "Quit Game"
    );

    commands.insert_resource(MenuEntities {
        button_play,
        button_load,
        button_save,
        button_cred,
        button_quit,
    });
}

/// Called when in MENU state
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
                    MenuButtonAction::Cred => {
                        next_state.set(AppState::Credits);
                    }
                    MenuButtonAction::Load => {
                        
                    }
                    MenuButtonAction::Play => {
                        next_state.set(AppState::NewGame);
                    }
                    MenuButtonAction::Save => {
                        
                    }
                    MenuButtonAction::Quit => {
                        let _ = app_exit_events.send(AppExit);
                        ()
                    }
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

/// Called when exiting MENU state
pub fn cleanup(mut commands: Commands, menu_entities: Res<MenuEntities>) {
    commands
        .entity(menu_entities.button_play)
        .despawn_recursive();
    commands
        .entity(menu_entities.button_load)
        .despawn_recursive();
    commands
        .entity(menu_entities.button_save)
        .despawn_recursive();
    commands
        .entity(menu_entities.button_cred)
        .despawn_recursive();
    commands
        .entity(menu_entities.button_quit)
        .despawn_recursive();
}
