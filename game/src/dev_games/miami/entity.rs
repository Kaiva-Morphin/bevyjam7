use std::f32::consts::PI;

use camera::CameraController;
use room::Focusable;

use crate::{dev_games::miami::{plugin::{MiamiAssets, STATE, miami_character_layers, miami_player_layers}, shadows::ShadowInit}, prelude::*};


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

#[derive(Component)]
pub struct CharacterComponents {
    pub pivot: Entity,
    pub sprite: Entity,
}


impl MiamiEntity {
    pub fn to_character(&self) -> CharacterSprite {
        CharacterSprite {
            default_rect: Rect::new(0.0, 16.0, 32.0, 32.0),
            default_offset: vec3(0.0, 0.0, 0.0),
            foot_offset: vec3(0.0, 0.0, 0.0),
            foot_rect: Rect::new(0.0, 0.0, 16.0, 16.0),
        }
    }
}

#[derive(Component)]
pub struct CharacterFoot {
    pub t: f32,
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

    let foot1 = cmd.spawn((
        Sprite {
            image: assets.character.clone(),
            rect: Some(char.foot_rect.clone()),
            ..Default::default()
        },
        Transform::from_xyz(-2., 0., -0.4),
        CharacterFoot { t: 0.0 },
    )).id();
    let foot2 = cmd.spawn((
        Sprite {
            image: assets.character.clone(),
            rect: Some(char.foot_rect.clone()),
            flip_x: true,
            ..Default::default()
        },
        Transform::from_xyz(2., 0., -0.4),
        CharacterFoot { t: 0.5 },
    )).id();


    let sprite = cmd.spawn((
        ShadowInit,
        Sprite {
            image: assets.character.clone(),
            rect: Some(char.default_rect.clone()),
            ..Default::default()
        },
        char
    )).id();
    let pivot = cmd.spawn((
        Name::new("Pivot"),
        CharacterPivotPoint,
        Visibility::default(),
        GlobalTransform::default(),
        Transform::default(),
    )).id();
    
    cmd.entity(pivot).add_children(&[foot1, foot2]);
    

    cmd.entity(pivot).add_child(sprite);
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
        CharacterController{speed: 80.0, ..Default::default()},
        Friction::ZERO,
        CollisionEventsEnabled,
        CharacterComponents{sprite, pivot},
    ));
    let id;
    if let MiamiEntity::Player = spawner.entity_type {
        id = c.insert(
            (Focusable, Player, miami_player_layers())
        ).id();
        camera_controller.focused_entities.push_front(id);
    } else {
        id = c.insert(miami_character_layers()).id();
    }
    cmd.entity(id).add_child(pivot);
}


#[derive(Component)]
pub struct CharacterSprite {
    pub default_rect: Rect,
    pub default_offset: Vec3,

    pub foot_rect: Rect,
    pub foot_offset: Vec3,
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
    mut entities: Query<(&CharacterController, &mut LinearVelocity, &CharacterComponents)>,
    mut pivots: Query<&mut Transform, With<CharacterPivotPoint>>,
    mut foots: Query<(&ChildOf, &mut Transform, &mut CharacterFoot), Without<CharacterPivotPoint>>,
    time: Res<Time>,
) {
    let dt = time.dt();
    for (controller, mut velocity, c) in entities.iter_mut() {
        let i = controller.input_dir.normalize_or_zero();
        velocity.x = i.x * controller.speed;
        velocity.y = i.y * controller.speed;
        let Ok(mut t) = pivots.get_mut(c.pivot) else {continue;};
        t.rotation = Quat::from_rotation_z(
            controller.look_dir.to_angle() + std::f32::consts::FRAC_PI_2
        );

        for (child, mut t, mut f) in foots.iter_mut() {
            if child.0 != c.pivot {continue;}
            if i.x == 0.0 && i.y == 0.0 {continue;}
            const AMP : f32 = 5.0;
            const SPD : f32 = 0.8;
            f.t += dt * SPD;
            t.translation.y = (f.t * PI * 2.0).sin() * AMP; 
            if f.t > 1.0 {f.t -= 1.0;}
        }
    }
}


