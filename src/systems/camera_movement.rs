use bevy::{
    prelude::*, //default bevy
    input::{keyboard::KeyCode, Input},
};

use crate::{
    AppState,
    UNIVERSE_SIZE,
    systems::life::LIFE_FORM_SIZE,
};

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        projection: PerspectiveProjection  {
            near: 0.1,
            far: 10000.0,
            aspect_ratio: 16.0/9.0,
            fov: std::f32::consts::FRAC_PI_3,
        }
        .into(),
        transform: Transform::from_xyz(0.0, 0.0, LIFE_FORM_SIZE*2.0*UNIVERSE_SIZE as f32).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()  
    });
}

const MOVE_SPEED: f32 = std::f32::consts::FRAC_1_PI/3.0;

pub fn move_camera_on_keyboard_input(
    mut camera: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    timer: Res<Time>,
    mut state: ResMut<State<AppState>>,
) {
    let move_factor = 1000.0 * timer.delta_seconds();
    //let rotation_factor = 500.0 * timer.delta_seconds();
    for mut transform in camera.iter_mut() {
        //rotation
        if keys.pressed(KeyCode::A) {
            // look left
            transform.rotate_local_y(MOVE_SPEED);
        } else if keys.pressed(KeyCode::D) {
            // look right
            transform.rotate_local_y(-MOVE_SPEED);
        }
        // forward / backward
        if keys.pressed(KeyCode::W) {
            // forward
            let move_cam = transform.forward() * move_factor;
            transform.translation += move_cam;
        } else if keys.pressed(KeyCode::S) {
            // backward
            let move_cam = transform.forward() * move_factor;
            transform.translation -= move_cam;
        }
        //movement
        if keys.pressed(KeyCode::Left) {
            // moving left
            transform.translation.x -= move_factor;
        } else if keys.pressed(KeyCode::Right) {
            // moving right
            transform.translation.x += move_factor;
        }
        if keys.pressed(KeyCode::Up) {
            // moving up
            transform.translation.y += move_factor;
        } else if keys.pressed(KeyCode::Down) {
            // moving down
            transform.translation.y -= move_factor;
        }
        if keys.just_pressed(KeyCode::Space) {
            // (un)pause game
            match state.current() {
                AppState::InGame => state.set(AppState::Paused).unwrap(),
                AppState::Paused => state.set(AppState::InGame).unwrap(),
                AppState::Splash => {},
            }
        }
        if keys.just_pressed(KeyCode::Return) {
            // todo, place life form?
        }
    }
}
