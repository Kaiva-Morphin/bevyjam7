use camera::CameraController;
use room::Focusable;

use crate::{dev_games::miami::{plugin::{MiamiAssets, STATE, miami_character_layers, miami_player_layers}, shadows::{ShadowCaster, ShadowInit, ShadowPivot}}, prelude::*};


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterPivotPoint;


#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct MiamiEntitySpawner {
    pub entity_type: MiamiEntity
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub enum MiamiEntity {
    #[default]
    Player,
    Dummy,
    Endoskeleton,
    PurpleGuy,
    Freddy,
}


impl MiamiEntity {
    pub fn to_character(&self) -> CharacterSprite {
        CharacterSprite {
            default_rect: Rect::new(0.0, 16.0, 32.0, 32.0),
            default_offset: vec3(0.0, 0.0, 0.0),
        }
    }
}



pub fn on_entity_spawnpoint(
    point: On<Add, MiamiEntitySpawner>,
    q: Query<(&MiamiEntitySpawner, &Transform)>,
    mut cmd: Commands,
    assets: Res<MiamiAssets>,
    state: Res<State<AppState>>,
    mut camera_controller: ResMut<CameraController>,
){
    if state.get() != &STATE {return;}
    let Ok((spawner, transform)) = q.get(point.entity) else {return;};
    let char = spawner.entity_type.to_character();
    let mut c = cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Character"),
        GlobalTransform::default(),
        transform.clone(),
        Visibility::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::circle(7.0),
        GravityScale(0.0),
        CharacterController{speed:100.0, ..Default::default()},
        Friction::ZERO,
        CollisionEventsEnabled,
        children![(
            Name::new("Pivot"),
            CharacterPivotPoint,
            Visibility::default(),
            GlobalTransform::default(),
            Transform::default(),
            children![(
                ShadowInit,
                Sprite {
                    image: assets.character.clone(),
                    rect: Some(char.default_rect.clone()),
                    ..Default::default()
                },
                char
            )],
        )],
    ));
    if let MiamiEntity::Player = spawner.entity_type {
        camera_controller.focused_entities.push_front(c.insert(
            (Focusable, Player, miami_player_layers())
        ).id());
    } else {
        c.insert(miami_character_layers());
    }
    
}


#[derive(Component)]
pub struct CharacterSprite {
    pub default_rect: Rect,
    pub default_offset: Vec3,
}


#[derive(Component, Default)]
pub struct CharacterController {
    pub input_dir: Vec2,
    pub speed: f32,
    pub look_dir: Vec2,
    pub shoot: bool,
    pub throw: bool,
}


pub fn update_controllers(
    mut entities: Query<(&CharacterController, &mut LinearVelocity, &Children)>,
    mut pivots: Query<(&mut Transform, &Children), With<CharacterPivotPoint>>,
    
    // mut transforms: Query<&mut Transform, With<ShadowCaster>>
) {
    for (controller, mut velocity, c) in entities.iter_mut() {
        let i = controller.input_dir.normalize_or_zero();
        velocity.x = i.x * controller.speed;
        velocity.y = i.y * controller.speed;
        for c in c.iter() {
            let Ok((mut t, c)) = pivots.get_mut(c) else {continue;};
            t.rotation = Quat::from_rotation_z(
                controller.look_dir.to_angle() + std::f32::consts::FRAC_PI_2
            )
            // for c in c.iter() {
                // let Ok(mut transform) = transforms.get_mut(c) else {continue;};
                // transform.rotation = Quat::from_rotation_z(
                    // controller.look_dir.to_angle() + std::f32::consts::FRAC_PI_2
                // )
            // }
        }
    }
}


