use bevy::{
    prelude::*, //default bevy
    input::{keyboard::KeyCode, Input},
};

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        projection: PerspectiveProjection  {
            near: 0.1,
            far: 10000.0,
            aspect_ratio: crate::ARENA_WIDTH / crate::ARENA_HEIGHT,
            fov: std::f32::consts::FRAC_PI_3,
        }
        .into(),
        transform: Transform::from_xyz(10.0, 10.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()  
    });
}

const MOVE_SPEED: f32 = std::f32::consts::FRAC_1_PI/3.0;

pub fn move_camera_on_keyboard_input(
    mut camera: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    timer: Res<Time>,
) {
    let move_factor = 100.0 * timer.delta_seconds();
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
            transform.translation.z -= move_factor;
        } else if keys.pressed(KeyCode::S) {
            // backward
            transform.translation.z += move_factor;
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
