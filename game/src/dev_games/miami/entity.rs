use std::f32::consts::PI;

use avian2d::math::Vector;
use camera::CameraController;
use room::Focusable;

use crate::{dev_games::miami::{plugin::{MiamiAssets, STATE, miami_character_layers, miami_player_layers, miami_seeker_shapecast_layer}, shadows::ShadowInit}, prelude::*};


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

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub enum MiamiEntity {
    #[default]
    Player,
    Endoskeleton,
    Bonnie,
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
        match self {
            MiamiEntity::Player => CharacterSprite {
                default_rect: Rect::new(0.0, 16.0, 32.0, 32.0),
                default_offset: vec3(0.0, 0.0, 0.0),
                foot_offset: vec3(0.0, 0.0, 0.0),
                foot_rect: Rect::new(0.0, 0.0, 16.0, 16.0),
            },
            MiamiEntity::Endoskeleton | MiamiEntity::Bonnie => CharacterSprite {
                default_rect: Rect::new(0.0, 16.0, 32.0, 32.0),
                default_offset: vec3(0.0, 0.0, 0.0),
                foot_offset: vec3(0.0, 0.0, 0.0),
                foot_rect: Rect::new(0.0, 0.0, 16.0, 16.0),
            },
            _ => todo!()

        }
    }
    pub fn to_handle(&self, assets: &Res<MiamiAssets>) -> Handle<Image> {
        match self {
            MiamiEntity::Player => assets.character.clone(),
            MiamiEntity::Endoskeleton => assets.endoskeleton.clone(),
            MiamiEntity::Bonnie => assets.bonnie.clone(),
            _ => todo!()
        }
    }
    pub fn to_chaser(&self, start: Vec2) -> ChaserAi {
        match self {
            MiamiEntity::Player => unimplemented!(),
            MiamiEntity::Endoskeleton => ChaserAi {
                seek_range: 300.0,
                attention_range: 100.0,
                origin_point: start,
                seek_time: 1.0,
                ..Default::default()
            },  
            MiamiEntity::Bonnie => ChaserAi{
                seek_range: 300.0,
                attention_range: 100.0,
                origin_point: start,
                seek_time: 20.0,
                ..Default::default()
            },
            _ => todo!()
        }
    }
}

impl CharacterController {
    fn from_type(entity_type: &MiamiEntity) -> Self {
        match entity_type {
            MiamiEntity::Player => Self {
                speed: 80.0,
                run_speed: 80.0,
                walk_speed: 80.0,
                hp: 1.0,
                max_hp: 1.0,
                ..Default::default()},
            MiamiEntity::Endoskeleton => Self {
                speed: 120.0, 
                run_speed: 120.0, 
                walk_speed: 60.0,
                hp: 100.,
                max_hp: 100.,
                ..Default::default()
            },
            MiamiEntity::Bonnie => Self {
                speed: 100.0,
                run_speed: 100.0,
                walk_speed: 30.0,
                hp: 300.,
                max_hp: 300.,
                ..Default::default()
            },
            _ => todo!()
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
            image: spawner.entity_type.to_handle(&assets),
            rect: Some(char.foot_rect.clone()),
            ..Default::default()
        },
        Transform::from_xyz(-2., 0., -0.4),
        CharacterFoot { t: 0.0 },
    )).id();
    let foot2 = cmd.spawn((
        Sprite {
            image: spawner.entity_type.to_handle(&assets),
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
            image: spawner.entity_type.to_handle(&assets),
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
    
    let mut controller = CharacterController::from_type(&spawner.entity_type);
    
    controller.look_dir = spawner.look_dir;

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
        controller,
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
        let chaser = spawner.entity_type.to_chaser(transform.translation.truncate());
        let caster = ShapeCaster::new(
            Collider::circle(4.0),
            Vector::ZERO,
            0.0,
            Dir2::NEG_Y
        )
            .with_max_distance(chaser.seek_range)
            .with_ignore_self(true)
            .with_query_filter(SpatialQueryFilter::from_mask(miami_seeker_shapecast_layer().memberships));
        id = c.insert((
            miami_character_layers(),
            caster,
            chaser,
        )).id();
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
    pub run_speed: f32,
    pub walk_speed: f32,
    pub max_hp: f32,
    
    pub input_dir: Vec2,
    pub speed: f32,
    pub look_dir: Vec2,

    pub shoot: bool,
    pub throw: bool,
    pub hp: f32,
}


#[derive(Component, Default)]
pub struct ChaserAi {
    pub seek_range: f32, // from front
    pub attention_range: f32, // from back
    pub last_seen: Option<Vec2>,
    pub origin_point: Vec2,
    pub seek_time: f32,
    pub max_seek_time: f32,
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
        t.rotation = Quat::from_rotation_z(
            controller.look_dir.to_angle() + std::f32::consts::FRAC_PI_2
        );

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


pub fn update_chasers(
    mut entities: Query<(&mut CharacterController, &mut LinearVelocity, &mut ChaserAi, &GlobalTransform, &ShapeHits, &mut ShapeCaster)>,
    player: Query<(Entity, &GlobalTransform), With<Player>>,
    // navmeshes: Res<Assets<NavMesh>>,
    // navmesh: Single<&NavMeshSettings>,
) {
    let Some((player, pt)) = player.iter().next() else {return;};
    for (mut controller, mut velocity, mut chaser, gt, hits, mut caster) in entities.iter_mut() {
        // let mut target = None;
        // for hit in hits.iter() {
        //     if let Ok(e) = entities.get(hit.entity) {target = Some(e.0);break;}
        // }
        let d = (pt.translation() - gt.translation());
        let nd = d.normalize_or_zero();
        caster.direction = Dir2::from_xy(nd.x, nd.y).expect("Not a dir");
        chaser.last_seen = None;
        for hit in hits {
            if hit.entity == player {
                controller.look_dir = nd.truncate();
                chaser.last_seen = Some(nd.truncate());
            }
        }
    }
}