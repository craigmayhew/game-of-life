use bevy::{
    prelude::*, //default bevy
};

use crate::{systems::life::LIFE_FORM_SIZE, DEFAULT_UNIVERSE_SIZE};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: PerspectiveProjection {
            near: 0.1,
            far: 10000.0,
            aspect_ratio: 16.0 / 9.0,
            fov: std::f32::consts::FRAC_PI_3,
        }
        .into(),
        transform: Transform::from_xyz(
            DEFAULT_UNIVERSE_SIZE as f32 * LIFE_FORM_SIZE / 2.0,
            DEFAULT_UNIVERSE_SIZE as f32 * LIFE_FORM_SIZE / 2.0,
            DEFAULT_UNIVERSE_SIZE as f32 * LIFE_FORM_SIZE * 2.0,
        )
        .looking_at(
            Vec3::new(
                DEFAULT_UNIVERSE_SIZE as f32 * LIFE_FORM_SIZE / 2.0,
                DEFAULT_UNIVERSE_SIZE as f32 * LIFE_FORM_SIZE / 2.0,
                0.0,
            ),
            Vec3::Y,
        ),
        ..default()
    });
}

const ROTATE_SPEED: f32 = std::f32::consts::FRAC_1_PI / 5.0;

pub fn move_camera_on_keyboard_input(
    mut camera: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    timer: Res<Time>,
) {
    let move_factor = 1000.0 * timer.delta_seconds();
    //let rotation_factor = 500.0 * timer.delta_seconds();
    for mut transform in camera.iter_mut() {
        //rotation
        if keys.pressed(KeyCode::A) {
            // look left
            transform.rotate_local_y(ROTATE_SPEED);
        } else if keys.pressed(KeyCode::D) {
            // look right
            transform.rotate_local_y(-ROTATE_SPEED);
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
            let move_cam = transform.left() * move_factor;
            transform.translation += move_cam;
        } else if keys.pressed(KeyCode::Right) {
            // moving right
            let move_cam = transform.right() * move_factor;
            transform.translation += move_cam;
        }
        if keys.pressed(KeyCode::Up) {
            // moving up
            transform.translation.y += move_factor;
        } else if keys.pressed(KeyCode::Down) {
            // moving down
            transform.translation.y -= move_factor;
        }
    }
}
