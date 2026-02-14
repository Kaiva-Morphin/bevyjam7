use std::f32::consts::PI;

use avian2d::math::Vector;
use bevy::{color::palettes, ecs::spawn};
use camera::CameraController;
use rand::Rng;
use room::Focusable;


use super::{plugin::{MiamiAssets, STATE, back_body_rect, blood_rects, front_body_rect, miami_character_layers, miami_player_layers, miami_seeker_shapecast_layer, oil_blood, red_blood}, shadows::ShadowInit, weapon::{ArmedCharacter, WeaponComponents, WeaponOf, WeaponSprite, WeaponType}};
use crate::{dev_games::miami::{bossfight::{BossFightStandAi, BossFightWait}, plugin::CHASER_RANDOM_RADIUS}, prelude::*};

#[derive(Component)]
pub struct InvincibleCharacter;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterPivotPoint;


#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct MiamiEntitySpawner {
    pub entity_type: MiamiEntity,
    pub look_dir: Vec2,
}

#[derive(Component, Default, Reflect, Eq, PartialEq, Debug, Clone)]
#[reflect(Component, Default)]
pub enum MiamiEntity {
    #[default]
    Player,
    Endoskeleton,
    GoldenEndoskeleton,
    CopperEndoskeleton,
    Bonnie,
    NewBonnie,
    Chicka,
    NewChicka,
    Freddy,
}

#[derive(Component)]
pub struct CharacterComponents {
    pub pivot: Entity,
    pub sprite: Entity,
}


impl MiamiEntity {
    pub fn to_character(&self) -> CharacterSprite {
        info!("To character: {:?}", self);
        match self {
            MiamiEntity::Player | MiamiEntity::Endoskeleton | MiamiEntity::Bonnie | MiamiEntity::Chicka
            | MiamiEntity::CopperEndoskeleton | MiamiEntity::NewBonnie | MiamiEntity::NewChicka
            | MiamiEntity::GoldenEndoskeleton 
            => CharacterSprite {
                default_rect: Rect::new(0.0, 16.0, 32.0, 32.0),
                default_offset: vec3(0.0, 0.0, 0.0),
                foot_offset: vec3(0.0, 0.0, 0.0),
                foot_rect: Rect::new(0.0, 0.0, 16.0, 16.0),
            },
            MiamiEntity::Freddy => CharacterSprite {
                default_rect: Rect::new(0.0, 0.0, 48.0, 48.0),
                default_offset: vec3(0.0, 11.0, 0.0),
                foot_offset: vec3(0.0, 0.0, 0.0),
                foot_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            },
            // _ => todo!()
        }
    }
    pub fn to_handle(&self, assets: &Res<MiamiAssets>) -> Handle<Image> {
        match self {
            MiamiEntity::Player => assets.character.clone(),
            MiamiEntity::Bonnie => assets.bonnie.clone(),
            MiamiEntity::NewBonnie => assets.new_bonnie.clone(),
            MiamiEntity::Chicka => assets.chica.clone(),
            MiamiEntity::NewChicka => assets.new_chica.clone(),
            MiamiEntity::Endoskeleton => assets.endoskeleton.clone(),
            MiamiEntity::CopperEndoskeleton => assets.copper_endoskeleton.clone(),
            MiamiEntity::GoldenEndoskeleton => assets.golden_endoskeleton.clone(),
            MiamiEntity::Freddy => assets.freddy.clone(),
            // _ => todo!()
        }
    }
    pub fn to_chaser(&self, start: Vec2) -> ChaserAi {
        match self {
            MiamiEntity::Player => unimplemented!(),
            MiamiEntity::Endoskeleton => ChaserAi {
                seek_range: 300.0,
                attention_range: 100.0,
                origin_point: start,
                max_seek_time: 1.0,
                max_stay_time: 10.0,
                ..Default::default()
            },  
            MiamiEntity::Bonnie => ChaserAi{
                seek_range: 300.0,
                attention_range: 100.0,
                origin_point: start,
                max_seek_time: 20.0,
                max_stay_time: 10.0,   
                ..Default::default()
            },
            _ => ChaserAi{ // TODO!
                seek_range: 300.0,
                attention_range: 100.0,
                origin_point: start,
                max_seek_time: 20.0,
                max_stay_time: 10.0,   
                ..Default::default()
            }
        }
    }
}

impl CharacterController {
    fn from_type(entity_type: &MiamiEntity) -> Self {
        let e = entity_type.clone();
        match entity_type {
            MiamiEntity::Player => Self {
                speed: 120.0,
                run_speed: 120.0,
                walk_speed: 120.0,
                hp: 1200.0,
                prev_hp: 1200.0,
                blood_rects: blood_rects(),
                blood_color: red_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            MiamiEntity::Endoskeleton => Self {
                speed: 100.0, 
                run_speed: 100.0, 
                walk_speed: 60.0,
                hp: 100.,
                prev_hp: 100.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            MiamiEntity::Bonnie => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                prev_hp: 300.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            MiamiEntity::NewBonnie => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                prev_hp: 300.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            MiamiEntity::Chicka => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                prev_hp: 300.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            MiamiEntity::NewChicka => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                prev_hp: 300.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            MiamiEntity::CopperEndoskeleton => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                prev_hp: 300.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            MiamiEntity::Freddy => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                prev_hp: 300.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
            _ => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                prev_hp: 300.,
                blood_rects: blood_rects(),
                blood_color: oil_blood(),
                character: e,
                front_body_rect: front_body_rect(),
                back_body_rect: back_body_rect(),
                ..Default::default()
            },
        }
        
    }
}

#[derive(Component)]
pub struct CharacterFoot {
    pub t: f32,
}

pub fn spawn_entity(
    cmd: &mut Commands,
    entity_type: MiamiEntity,
    assets: &Res<MiamiAssets>,
    camera_controller: &mut ResMut<CameraController>,
    mut transform : Transform,
    look_dir : Vec2
) {
    transform.translation.z = -2.0;
    let char = entity_type.to_character();
    let foot1 = cmd.spawn((
        Sprite {
            image: entity_type.to_handle(assets),
            rect: Some(char.foot_rect.clone()),
            ..Default::default()
        },
        Transform::from_xyz(-2., 0., -0.4),
        CharacterFoot { t: 0.0 },
    )).id();
    let foot2 = cmd.spawn((
        Sprite {
            image: entity_type.to_handle(assets),
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
            image: entity_type.to_handle(&assets),
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
    
    let mut controller = CharacterController::from_type(&entity_type);
    
    controller.look_dir = look_dir;

    cmd.entity(pivot).add_child(sprite);
    let mut c = cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Character"),
        GlobalTransform::default(),
        Visibility::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::circle(7.0),
        GravityScale(0.0),
        controller,
        Friction::ZERO,
        CollisionEventsEnabled,
        CharacterComponents{sprite, pivot},
    ));
    let id;
    if let MiamiEntity::Player = entity_type {
        id = c.insert(
            (
                transform.clone(),
                Focusable,
                Player,
                // super::player::PlayerDisabled, // ! ########
                Name::new("Player"),
                miami_player_layers()
            )
        ).id();
        camera_controller.focused_entities.push_front(id);
    } else {
        let chaser = entity_type.to_chaser(transform.translation.truncate());
        let caster = ShapeCaster::new(
            Collider::circle(1.0),
            Vector::ZERO,
            0.0,
            Dir2::NEG_Y
        )
            .with_max_distance(chaser.seek_range)
            .with_ignore_self(true)
            .with_query_filter(SpatialQueryFilter::from_mask(miami_seeker_shapecast_layer().memberships));
        id = c.insert((
            transform.clone(),
            miami_character_layers(),
            CharacterInPlace,
            // DummyEntity,
            caster,
            chaser,
        )).id();

        let weapon;
        match entity_type {
            MiamiEntity::NewBonnie => {weapon = WeaponType::BonniePlay.to_weapon();},
            MiamiEntity::NewChicka => {weapon = WeaponType::ChickaThrow.to_weapon();},
            MiamiEntity::Freddy => {weapon = WeaponType::FazStar.to_weapon();},
            _ => weapon = WeaponType::EnemyFists.to_weapon()
        }

        let sprite = cmd.spawn((
            Sprite {
                image: assets.weapons.clone(),
                rect: Some(weapon.rect.clone()),
                ..Default::default()
            },
            Transform::default(),
            WeaponSprite
        )).id();
        let weapon = cmd.spawn((
            weapon,
            DespawnOnExit(STATE),
            Transform::default(),
            WeaponComponents{sprite},
            WeaponOf(id),
        )).add_child(sprite).id();
        cmd.entity(
            id
        ).insert(
            ArmedCharacter(weapon),
        ).add_child(weapon);
    }
    if entity_type == MiamiEntity::NewBonnie 
    || entity_type == MiamiEntity::NewChicka
    || entity_type == MiamiEntity::Freddy 
    {
        cmd.entity(id).insert((InvincibleCharacter, BossFightWait));
    }
    cmd.entity(id).add_child(pivot);
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
    spawn_entity(&mut cmd, spawner.entity_type.clone(), &assets, &mut camera_controller, transform.clone(), spawner.look_dir);
}


#[derive(Component)]
pub struct CharacterSprite {
    pub default_rect: Rect,
    pub default_offset: Vec3,

    pub foot_rect: Rect,
    pub foot_offset: Vec3,
}


#[derive(Component)]
pub struct CharacterController {
    pub run_speed: f32,
    pub walk_speed: f32,
    pub prev_hp: f32,
    
    pub input_dir: Vec2,
    pub speed: f32,
    pub look_dir: Vec2,

    pub shoot: bool,
    pub throw: bool,
    pub hp: f32,

    pub blood_rects: [Rect; 3],
    pub blood_color: Color,

    pub character: MiamiEntity,
    pub front_body_rect: Rect,
    pub back_body_rect: Rect,
    pub body_offset: Vec3,

    pub last_impact_back: bool,
    pub last_impact_dir: Vec2,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            run_speed: 120.0,
            walk_speed: 120.0,
            prev_hp: 1.0,
            input_dir: Vec2::ZERO,
            speed: 120.0,
            look_dir: Vec2::ZERO,
            shoot: false,
            throw: false,
            hp: 1.0,
            blood_rects: blood_rects(),
            blood_color: oil_blood(),
            character: MiamiEntity::Endoskeleton,
            front_body_rect: front_body_rect(),
            back_body_rect: back_body_rect(),
            last_impact_back: false,
            last_impact_dir: Vec2::NEG_Y,
            body_offset: vec3(0.0, 22.0, 0.0),
        }
    }
}



#[derive(Component, Default)]
pub struct ChaserAi {
    pub seek_range: f32, // from front
    pub attention_range: f32, // from back
    pub last_seen: Option<Vec2>,
    pub origin_dir: Vec2,
    pub origin_point: Vec2,
    pub seek_time: f32,
    pub max_seek_time: f32,

    pub stay_time: f32,
    pub max_stay_time: f32,
}


#[derive(Component)]
pub struct DummyEntity;


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
        if controller.look_dir != Vec2::ZERO {
            t.rotation = Quat::from_rotation_z(controller.look_dir.to_angle() + std::f32::consts::FRAC_PI_2);      
        }
        for (child, mut t, mut f) in foots.iter_mut() {
            if child.0 != c.pivot {continue;}
            if i.x == 0.0 && i.y == 0.0 {continue;}
            const AMP : f32 = 5.0;
            const SPD : f32 = 0.012;
            f.t += dt * SPD * controller.speed;
            t.translation.y = (f.t * PI * 2.0).sin() * AMP; 
            if f.t > 1.0 {f.t -= 1.0;}
        }
    }
}

#[derive(Component)]
pub struct Path {
    current: Vec3,
    next: Vec<Vec3>,
}

pub fn update_chasers(
    mut entities: Query<
        (
            Entity, &mut CharacterController, &mut ChaserAi, 
            &GlobalTransform, &ShapeHits, &mut ShapeCaster
        ), 
        (
            Without<DummyEntity>, Without<BossFightWait>, Without<BossFightStandAi>
        )>,
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    navmeshes: Res<Assets<NavMesh>>,
    navmesh: Query<&ManagedNavMesh>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    let dt = time.dt();
    let mut rand = rand::rng();
    let Some(mesh) = navmesh.iter().last() else {return;};
    let Some(navmesh) = navmeshes.get(mesh) else {return;};
    let Some((player, pt)) = player.iter().next() else {return;};
    for (e, mut controller, mut chaser, gt, hits, mut caster) in entities.iter_mut() {
        let d = pt.translation() - gt.translation();
        let nd = d.normalize_or_zero();
        let Ok(dir) = Dir2::from_xy(nd.x, nd.y) else {continue;};
        caster.direction = dir;
        let mut last_seen = None;
        let mut max_dist = caster.max_distance;
        let mut t = gt.translation();
        if nd.truncate().dot(controller.look_dir) < 0.0 {
            max_dist = chaser.attention_range;
        }
        for hit in hits {
            if hit.entity == player && hit.distance < max_dist {
                controller.look_dir = nd.truncate();
                last_seen = Some(pt.translation().truncate());
            }
        }
        if let Some(last_seen) = last_seen {
            if chaser.last_seen != Some(last_seen) {
                cmd.entity(e).remove::<CharacterInPlace>();
                t.z = 0.0;
                let Some(path) = navmesh.transformed_path(t, last_seen.extend(0.0)) else {
                    continue;
                };
                let Some((f, r)) = path.path.split_first() else {continue;};
                let mut remaining = r.to_vec();
                chaser.last_seen = Some(last_seen);
                remaining.reverse();
                controller.speed = controller.run_speed;
                cmd.entity(e).insert(
                    Path {
                        current: *f,
                        next: remaining
                    }
                );
            }
        } else if chaser.seek_time > chaser.max_seek_time {
            chaser.seek_time = 0.0;
            chaser.last_seen = None;
            controller.look_dir = Vec2::ZERO;
            let Some(path) = navmesh.transformed_path(t, chaser.origin_point.extend(0.0)) else {
                continue;
            };
            let Some((f, r)) = path.path.split_first() else {continue;};
            let mut remaining = r.to_vec();
            remaining.reverse();
            controller.speed = controller.walk_speed;
            cmd.entity(e).insert(
                Path {
                    current: *f,
                    next: remaining
                }
            );
        } else if chaser.stay_time >= chaser.max_stay_time {
            // info!("Seeking!");
            let mut pos = t + Vec3::new(rand.random_range(-CHASER_RANDOM_RADIUS..CHASER_RANDOM_RADIUS), rand.random_range(-CHASER_RANDOM_RADIUS..CHASER_RANDOM_RADIUS), 0.0);
            pos.z = 0.0;
            let Some(path) = navmesh.transformed_path(t, pos) else {
                continue;
            };
            chaser.last_seen = Some(pos.truncate());
            let Some((f, r)) = path.path.split_first() else {continue;};
            let mut remaining = r.to_vec();
            chaser.stay_time = 0.0;
            remaining.reverse();
            controller.speed = controller.walk_speed;
            cmd.entity(e).insert(
                Path {
                    current: *f,
                    next: remaining
                }
            ).remove::<CharacterInPlace>();
        } else {
            chaser.stay_time += dt;
        }
    }
}


#[derive(Component)]
pub struct CharacterInPlace;

pub fn chase(
    mut commands: Commands,
    mut navigator: Query<(
        &mut Transform,
        Option<&mut Path>,
        &mut ChaserAi,
        Entity,
        &mut CharacterController
    ), (Without<Player>, Without<CharacterInPlace>)>,
    time: Res<Time>,
){
    let dt = time.dt();
    for (transform, path, mut chaser, entity, mut controller) in navigator.iter_mut() {
        let Some(mut path) = path else {
            controller.input_dir = Vec2::ZERO;
            // controller.look_dir = Vec2::ZERO;
            // controller.shoot = false;
            controller.shoot = false;
            chaser.seek_time += dt;
            continue;
        };
        let move_direction = path.current - transform.translation;
        controller.input_dir = move_direction.normalize_or_zero().truncate();
        if chaser.last_seen.is_none() {
            controller.look_dir = controller.input_dir;
        } else {
            controller.shoot = true;
        }

        if transform.translation.distance(path.current) < 5.0 {
            if let Some(next) = path.next.pop() {
                path.current = next;
            }
        }
        if transform.translation.distance(path.current) < 10.0 && path.next.is_empty() {
            chaser.seek_time = 0.0;
            if chaser.origin_point.distance(transform.translation.truncate()) < 10.0 {
                commands.entity(entity).insert(CharacterInPlace);
                controller.look_dir = chaser.origin_dir;
                controller.input_dir = Vec2::ZERO;
            }
            commands
                .entity(entity)
                .remove::<Path>();
            continue;
        }
    }
}


pub fn display_path(navigator: Query<(&Transform, &Path)>, mut gizmos: Gizmos) {
    for (transform, path) in &navigator {
        let mut to_display = path.next.iter().map(|v| v.xy()).collect::<Vec<_>>();
        to_display.push(path.current.xy());
        to_display.push(transform.translation.xy());
        to_display.reverse();
        if !to_display.is_empty() {
            gizmos.linestrip_2d(to_display, palettes::tailwind::YELLOW_400);
        }
    }
}
