use bevy::{
    core_pipeline::{
        bloom::BloomSettings,     // for bloom
        tonemapping::Tonemapping, // for bloom
    },
    prelude::*,                                           //default bevy
    render::camera::{Exposure, PhysicalCameraParameters}, // camera exposure added in bevy 0.13
};

use crate::{systems::life::LIFE_FORM_SIZE, DEFAULT_UNIVERSE_SIZE};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // Using a tonemapper that desaturates to white is recommended
            projection: PerspectiveProjection {
                near: 0.1,
                far: 100_000_000.0,
                aspect_ratio: 16.0 / 9.0,
                fov: std::f32::consts::FRAC_PI_3,
            }
            .into(),
            exposure: Exposure::from_physical_camera(PhysicalCameraParameters {
                aperture_f_stops: 1.0,
                shutter_speed_s: 1.0 / 100.0,
                sensitivity_iso: 100.0,
            }),
            transform: Transform::from_xyz(
                (DEFAULT_UNIVERSE_SIZE << 1) as f32 * -LIFE_FORM_SIZE,
                (DEFAULT_UNIVERSE_SIZE >> 1) as f32 * LIFE_FORM_SIZE,
                (DEFAULT_UNIVERSE_SIZE << 1) as f32 * LIFE_FORM_SIZE,
            )
            .looking_at(
                Vec3::new(
                    5_000_000.0,
                    (DEFAULT_UNIVERSE_SIZE >> 1) as f32 * LIFE_FORM_SIZE,
                    0.0,
                ),
                Vec3::Y,
            ),
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}

const ROTATE_SPEED: f32 = std::f32::consts::FRAC_1_PI / 5.0;

pub fn move_camera_on_keyboard_input(
    mut camera: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    let move_factor = 1000.0 * timer.delta_seconds();
    //let rotation_factor = 500.0 * timer.delta_seconds();
    for mut transform in camera.iter_mut() {
        //rotation
        if keys.pressed(KeyCode::KeyA) {
            // look left
            transform.rotate_local_y(ROTATE_SPEED);
        } else if keys.pressed(KeyCode::KeyD) {
            // look right
            transform.rotate_local_y(-ROTATE_SPEED);
        }
        // forward / backward
        if keys.pressed(KeyCode::KeyW) {
            // forward
            let move_cam = transform.forward() * move_factor;
            transform.translation += move_cam;
        } else if keys.pressed(KeyCode::KeyS) {
            // backward
            let move_cam = transform.forward() * move_factor;
            transform.translation -= move_cam;
        }
        //movement
        if keys.pressed(KeyCode::ArrowLeft) {
            // moving left
            let move_cam = transform.left() * move_factor;
            transform.translation += move_cam;
        } else if keys.pressed(KeyCode::ArrowRight) {
            // moving right
            let move_cam = transform.right() * move_factor;
            transform.translation += move_cam;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            // moving up
            transform.translation.y += move_factor;
        } else if keys.pressed(KeyCode::ArrowDown) {
            // moving down
            transform.translation.y -= move_factor;
        }
    }
}
