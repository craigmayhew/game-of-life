use amethyst::{
    core::{Time, Transform},
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
};

#[derive(Default)]
pub struct CameraMovementSystem {}

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut transforms, input_handler, time): Self::SystemData) {
        //delta_real_seconds() allows us to move at a consistent speed, irrespective of the frame rate
        let delta_time = time.delta_real_seconds();
        let move_factor = 1000.0 * delta_time;
        for (_, transform) in (&cameras, &mut transforms).join() {
            // move up / down
            if input_handler.action_is_down("CameraMoveUp").unwrap_or(false) {
                transform.move_up(move_factor);
            } else if input_handler.action_is_down("CameraMoveDown").unwrap_or(false) {
                transform.move_down(move_factor);
            }
            // move left /right
            if input_handler.action_is_down("CameraMoveLeft").unwrap_or(false) {
                transform.move_left(move_factor);
            } else if input_handler.action_is_down("CameraMoveRight").unwrap_or(false) {
                transform.move_right(move_factor);
            }
            // move forward / backward
            if input_handler.action_is_down("CameraMoveForward").unwrap_or(false) {
                transform.move_forward(move_factor);
            } else if input_handler.action_is_down("CameraMoveBackward").unwrap_or(false) {
                transform.move_backward(move_factor);
            }
            // look left / right
            if input_handler.action_is_down("CameraLookLeft").unwrap_or(false) {
                transform.append_rotation_y_axis(0.02);
            } else if input_handler.action_is_down("CameraLookRight").unwrap_or(false) {
                transform.append_rotation_y_axis(-0.02);
            }
        }
    }
}