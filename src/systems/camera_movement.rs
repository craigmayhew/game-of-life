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
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn move_camera_on_keyboard_input(
    mut camera: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
        // moving left
    } else if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
        // moving right
    }
    if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
        // moving up
    } else if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
        // moving down
    }
    if keys.just_pressed(KeyCode::Space) {
        // todo, place a life form?
    }
}
