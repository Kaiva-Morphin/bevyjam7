use bevy::input::{ButtonState, mouse::MouseButtonInput};

use crate::{dev_games::miami::entity::*, prelude::*};


pub fn control_player(
    mut player: Single<&mut CharacterController, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let mut c = player.into_inner();

    c.input_dir = Vec2::ZERO;
    c.throw = false;
    c.shoot = false;

    if keyboard_input.pressed(KeyCode::KeyA) {
        c.input_dir.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        c.input_dir.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        c.input_dir.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        c.input_dir.y -= 1.0;
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        c.shoot = true;
    }

    if mouse_input.just_pressed(MouseButton::Right) {
        c.throw = true;
    }
}

pub fn player_look_at_cursor(
    mut player: Single<(&mut CharacterController, &GlobalTransform), (With<Player>, Without<Camera>)>,
    window: Single<&Window>,
    outer_camera_q: Single<(&Camera, &GlobalTransform), With<HighresCamera>,>,
    world_camera_q: Single<&GlobalTransform, With<WorldCamera>,>,
) {
    let (camera, camera_transform) = *outer_camera_q;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        let p = cursor_world_pos + world_camera_q.translation().truncate();
        let (c, t) = &mut *player;
        c.look_dir = (p - t.translation().truncate()).normalize_or_zero();
    }
}
