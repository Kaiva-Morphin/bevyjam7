
use crate::prelude::*;


pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_player, tick_controllers).chain());
    }
}

#[derive(Component, Default)]
pub struct CharacterController {
    input_dir: Vec2,
    _picked_item: Option<Entity>,
}


pub fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut CharacterController, With<Player>>,
){
    let mut dir = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    for mut c in player.iter_mut() {
        c.input_dir = dir;
    }
}

pub fn tick_controllers(
    time: Res<Time>,
    mut player: Query<(&mut CharacterController, &mut LinearVelocity), With<Player>>,
){
    let dt = time.delta_secs().max(MAX_DT);
    for (c, mut t) in player.iter_mut() {
        t.x = c.input_dir.x * 2000.0 * dt;
        t.y = c.input_dir.y * 2000.0 * dt;
    }
}