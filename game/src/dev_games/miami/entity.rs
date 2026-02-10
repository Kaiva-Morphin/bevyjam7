use crate::prelude::*;


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterPivotPoint;

#[derive(Component)]
pub struct ShadowOf(pub Entity);


#[derive(Component)]
pub struct ShadowInit;


#[derive(Component)]
pub struct ShadowCaster;


#[derive(Component)]
pub struct Weapon {
    pub rect: Rect,
    pub offset: Vec3,
    
    pub char_rect: Rect,
    pub char_offset: Vec3
}


#[derive(Component)]
pub struct CharacterSprite {
    pub default_rect: Rect,
    pub default_offset: Vec3,
}


pub fn default_character() -> CharacterSprite {
    CharacterSprite {
        default_rect: Rect::new(0.0, 16.0, 32.0, 32.0),
        default_offset: vec3(0.0, 0.0, 0.0),
    }
}


pub fn setup_shadows(
    mut cmd: Commands,
    q: Query<(Entity, &ChildOf, &Sprite), With<ShadowInit>>
) {
    for (e, c, s)  in q.iter() {
        let mut s = s.clone();
        s.color = miami_shadow_color();
        let shadow = cmd.spawn((
            Name::new("Shadow"),
            s,
            Transform::from_translation(MIAMI_SHADOW_OFFSET),
            ShadowOf(e),
        )).id();
        cmd.entity(e).remove::<ShadowInit>().insert(ShadowCaster);
        cmd.entity(c.parent()).add_child(shadow);
    }
}


pub fn update_shadows(
    shadow_q: Query<(&mut Sprite, &mut Transform, &ShadowOf), (With<ShadowOf>, Without<ShadowCaster>)>,
    caster_q: Query<(&Sprite, &Transform), (With<ShadowCaster>, Without<ShadowOf>)>,
){
    for (mut sprite, mut transform, shadow) in shadow_q {
        let Ok((s, t)) = caster_q.get(shadow.0) else {continue};
        transform.rotation = t.rotation;
        sprite.rect = s.rect.clone();
    }
}


pub fn cleanup_shadows(
    mut cmd: Commands,
    shadows: Query<(Entity, &ShadowOf)>,
    casters: Query<Entity, With<ShadowCaster>>
) {
    for (e, shadow) in shadows {
        let Ok(_caster) = casters.get(shadow.0) else {continue;};
        cmd.entity(e).despawn();
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


#[derive(Component)]
pub struct CharacterController {
    pub input_dir: Vec2,
    pub speed: f32,
    pub look_dir: Vec2
}


pub fn update_controllers(
    mut entities: Query<(&CharacterController, &mut LinearVelocity, &Children)>,
    mut pivots: Query<&Children, With<CharacterPivotPoint>>,
    mut transforms: Query<&mut Transform, With<ShadowCaster>>
) {
    for (controller, mut velocity, c) in entities.iter_mut() {
        let i = controller.input_dir.normalize_or_zero();
        velocity.x = i.x * controller.speed;
        velocity.y = i.y * controller.speed;
        for c in c.iter() {
            let Ok(mut c) = pivots.get_mut(c) else {continue;};
            for c in c.iter() {
                let Ok(mut transform) = transforms.get_mut(c) else {continue;};
                transform.rotation = Quat::from_rotation_z(
                    controller.look_dir.to_angle() + std::f32::consts::FRAC_PI_2
                )
            }
        }
    }
}


pub fn control_player(
    mut player: Single<(&mut CharacterController), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut c) = player else {return;};
    c.input_dir = Vec2::ZERO;

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
}



