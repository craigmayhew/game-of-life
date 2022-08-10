use bevy::{
    prelude::*, //default bevy
    input::{keyboard::KeyCode, Input},
    render::camera::ScalingMode,
};

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 0.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

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
        } else if keys.pressed(KeyCode::D) {
            // look right
        }
        // forward / backward
        if keys.pressed(KeyCode::W) {
            // forward
            transform.translation.z += move_factor;
        } else if keys.pressed(KeyCode::S) {
            // backward
            transform.translation.z -= move_factor;
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
            // todo, pause game
        }
        if keys.just_pressed(KeyCode::Return) {
            // todo, place life form?
        }
    }
}
