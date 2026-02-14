use std::{collections::HashSet, time::Duration};

// use crate::{properties::{AppState, LastScreenshot, LastState}, prelude::*};
use crate::{hints::{HintAssets, KeyHint}, prelude::*};
use super::map::*;
use avian2d::math::Vector;
use bevy_asset_loader::asset_collection::AssetCollection;
use room::{Focusable, RoomController, on_room_spawned};
use camera::CameraController;
use games::global_music::plugin::NewBgMusic;


const STATE: AppState = AppState::Platformer;
const NEXT_STATE: AppState = AppState::FakeEnd;

pub struct PlatformerPlugin;

impl Plugin for PlatformerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(STATE), (
                setup,
            ))
            .add_systems(Update, (
                tick, swap, handle_enemy_collisions,
                frog_movement, frog_atlas_handler,
                toster_atlas_handler, toster_movement
            ).run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup)
            .add_systems(OnEnter(STATE), spawn_enemies)
            .register_type::<NextTrigger>()
            .register_type::<StopTrigger>()
            .register_type::<PlatformerSwitchableLayer>()
            .add_observer(focus_player)
            .add_observer(on_collision)
            .add_observer(on_room_spawned)
            .add_observer(on_stop_spawned)
            .add_observer(on_next_spawned)
            .add_observer(on_collider_spawned)
            ;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default, Reflect)]
#[reflect(Default, Component)]
struct NextTrigger;

#[derive(Component, Default, Reflect)]
#[reflect(Default, Component)]
struct StopTrigger;


#[derive(Component, Default, Reflect)]
#[reflect(Default, Component)]
struct SpawnPoint;

#[derive(AssetCollection, Resource)]
pub struct PlatformerAssets {
    #[asset(path = "maps/platformer/map.tmx")]
    tilemap: Handle<TiledMapAsset>,
    #[asset(path = "maps/platformer/character.png")]
    character: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 48, tile_size_y = 64, columns = 2, rows = 4))]
    character_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "maps/platformer/boneca_ambalabu.png")]
    frog: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 52, tile_size_y = 52, columns = 8, rows = 1))]
    frog_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "maps/platformer/ll_cacto_hipopotamo.png")]
    cactus: Handle<Image>,
    #[asset(path = "maps/platformer/rhino_tosterino.png")]
    toster: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 72, tile_size_y = 62, columns = 3, rows = 1))]
    toster_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "sounds/platformer/Three Red Hearts - Penguins vs Rabbits.ogg")]
    bg_music: Handle<AudioSource>,
}

#[derive(Component)]
pub struct PlayerSwitchSensor(pub Entity);

fn focus_player(
    point: On<Add, SpawnPoint>,
    state: Res<State<AppState>>,
    assets: Res<PlatformerAssets>,
    spawnpoint_q: Query<&Transform, (With<SpawnPoint>, Without<WorldCamera>)>,
    mut cq: Query<(Entity, &mut Projection), (With<WorldCamera>, Without<Player>)>,
    mut cmd: Commands,
    mut camera_controller: ResMut<CameraController>,
) {
    if state.get() != &STATE {return;}
    let Ok(pt) = spawnpoint_q.get(point.entity) else {return;};
    let pt = pt.translation;

    let collider = Collider::capsule(20.0, 20.0);
    let caster_shape = Collider::capsule(16.0, 16.0);
    let blocker_shape = Collider::capsule(12.0, 12.0);
    
    let switch = cmd.spawn((
        Sensor,
        CollisionEventsEnabled,
        ShapeCaster::new(blocker_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(8.1)
                .with_ignore_self(true)
                .with_query_filter(SpatialQueryFilter::from_mask(platformer_white_layer().filters & 0b110001)),
    )).id();
    let player = cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Player"),
        Sprite {
            image: assets.character.clone(),
            texture_atlas: Some(TextureAtlas{
                layout: assets.character_layout.clone(),
                index: 0,
            }),
            color: player_color_yellow(),
            ..default()
        },
        Player,
        platformer_player_yellow_layer(),
        RigidBody::Dynamic,
        LockedAxes::new().lock_rotation(),
        collider,
        CollisionEventsEnabled,
        Focusable,
        GravityScale(PLATFORMER_GRAVITY_FORCE),
        Transform::from_translation(pt),
        PlayerSwitchSensor(switch),
        ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(8.1)
                .with_ignore_self(true)
                .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Default)),
        Friction::new(0.0),
    )).id();
    cmd.entity(player).add_child(switch);
    camera_controller.focused_entities.push_front(player);
    let Some((ce, mut p)) = cq.iter_mut().next() else {return;}; 
    let Projection::Orthographic(p) = &mut *p else {warn!("Camera without perspective projection"); return;};
    p.scale = camera_controller.target_zoom;
    cmd.entity(ce).insert(Transform::from_translation(pt));
}


fn setup(
    mut cmd: Commands,
    assets: Res<PlatformerAssets>,
    mut latest: ResMut<LastState>,
    hint_assets: Res<HintAssets>,
    cam: Query<Entity, With<WorldCamera>>,
) {
    let cam = cam.iter().next().expect("No cam!");
    crate::hints::show_hints(
        &mut cmd,
        vec![KeyHint::KeysQAD, KeyHint::KeysSpace],
        STATE,
        cam,
        hint_assets,
    );
    cmd.spawn((
        NewBgMusic{handle: Some(assets.bg_music.clone()), instant_translation: false},
    ));
    latest.state = STATE;
    cmd.insert_resource(RoomController::default());

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Map"),
        TiledMap(assets.tilemap.clone()),
    ));
}


#[derive(Component)]
struct Disabled;

fn tick (
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (Entity, &mut LinearVelocity, &ShapeHits, &mut Sprite),
        (With<Player>, Without<Disabled>),
    >,
    sensors: Query<&Sensor>,
    mut t: Local<f32>,
) {
    let dt = time.dt();
    let mut grounded = false;
    for (_entity, mut linvel, hits, mut sprite) in &mut query {
        for hit in hits {
            if sensors.get(hit.entity).is_err() {
                grounded = true;
                break;
            }
        }
        if keyboard_input.pressed(KeyCode::Space) && grounded {
            linvel.y = PLATFORMER_JUMP_FORCE;
        }
        let s = if grounded {PLATFORMER_GROUND_GAIN} else {PLATFORMER_AIR_GAIN};
        let mut target = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) {
            target -= PLATFORMER_MAX_SPEED;
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            target += PLATFORMER_MAX_SPEED;
            sprite.flip_x = false;
        }
        *t += dt;
        if *t > PLATFORMER_ANIM_DELAY * 4.0 {
            *t = 0.0;
        }
        if grounded && let Some(ta) = &mut sprite.texture_atlas {
            if linvel.x.abs() < 2.0 {
                let i = (*t / (PLATFORMER_ANIM_DELAY * 2.0)).floor() as usize * 2;
                ta.index = i;
            } else {
                let i = (*t / (PLATFORMER_ANIM_DELAY)).floor() as usize * 2 + 1;
                ta.index = i;
            }
        }
        if !grounded && let Some(ta) = &mut sprite.texture_atlas {
            if linvel.y > 0.0 {
                ta.index = 4;
            } else {
                ta.index = 6;
            }
        }
        if grounded || target != 0.0 {
            linvel.x = linvel.x.move_towards(target, s * dt);
        }
    }
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
) {
    cmd.remove_resource::<RoomController>();
    cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
}

fn on_collision(
    _e: On<CollisionStart>,
    state: Res<State<AppState>>,
    mut cmd: Commands,
    p_q: Query<(Entity, &Position), With<Player>>,
    e_q: Query<&GlobalTransform, With<StopTrigger>>,
    n_q: Query<&NextTrigger>,
    // canvas: Res<camera::ViewportCanvas>,
    mut screenshot: ResMut<LastScreenshot>,
) {
    if state.get() != &STATE {return;}
    let e = _e.collider1;
    let p = _e.collider2;
    let Ok((p, t)) = p_q.get(p) else {return;};
    let is_next = n_q.get(e).is_ok();
    if is_next {
        if screenshot.awaiting == false {
            cmd.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(await_screenshot_and_translate(NEXT_STATE))
                ;
            screenshot.awaiting = true;
        }
    }
    let Ok(st) = e_q.get(e) else {return;};
    let x = st.translation().x;
    let y = t.y;
    cmd.entity(p).insert((LinearVelocity::ZERO, Disabled, Position::new(vec2(x, y))));
}

fn on_next_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    spawn_query: Query<&NextTrigger>,
    parents: Query<&ChildOf>,
    mut cmd: Commands,
    state: Res<State<AppState>>,
) {
    if state.get() != &STATE {return;}
    let spawn_entity = collider_created.event().origin;
    let Ok(p) = parents.get(spawn_entity) else {return;};
    let Ok(_nt) = spawn_query.get(p.parent()) else {return;};
    cmd.entity(spawn_entity).insert((
        DespawnOnExit(STATE),
        Name::new("Stop"),
        Sensor,
        NextTrigger,
        RigidBody::Static,
        CollisionEventsEnabled,
    ));
}

fn on_stop_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    spawn_query: Query<&StopTrigger>,
    parents: Query<&ChildOf>,
    mut cmd: Commands,
    state: Res<State<AppState>>,
) {
    if state.get() != &STATE {return;}
    let spawn_entity = collider_created.event().origin;
    let Ok(p) = parents.get(spawn_entity) else {return;};
    let Ok(_st) = spawn_query.get(p.parent()) else {return;};
    cmd.entity(spawn_entity).insert((
        DespawnOnExit(STATE),
        Name::new("Stop"),
        Sensor,
        StopTrigger,
        RigidBody::Static,
        CollisionEventsEnabled,
    ));
}

pub fn on_collider_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    p_q: Query<&ChildOf, With<ColliderOf>>,
    mut commands: Commands,
    assets: Res<Assets<TiledMapAsset>>,
    state: Res<State<AppState>>,
) { 
    if state.get() != &STATE {return;};
    let Some(layer) = collider_created.get_layer(&assets) else {return;};
    let Ok(p) = p_q.get(collider_created.event().origin) else {return;};
    let yellow = match layer.name.as_str() {
        "yellow" => true,
        "white" => false,
        _ => return
    };
    
    let layer = if yellow {platformer_yellow_layer()} else {platformer_white_layer()};
    commands.entity(collider_created.event().origin).insert(layer);
    if !yellow {
        commands.entity(p.parent()).insert(Visibility::Hidden);
    }
}

pub fn platformer_player_yellow_layer() ->       CollisionLayers {CollisionLayers::from_bits(0b0100111, 0b0100111)}
pub fn platformer_player_white_layer() ->        CollisionLayers {CollisionLayers::from_bits(0b0010111, 0b0010111)}
pub fn platformer_enemy_layer() ->               CollisionLayers {CollisionLayers::from_bits(0b0001001, 0b0001001)}
pub fn platformer_raycast_layer() ->             CollisionLayers {CollisionLayers::from_bits(0b0000000, 0b0110000)}
pub fn platformer_yellow_layer() ->              CollisionLayers {CollisionLayers::from_bits(0b0100000, 0b0100000)}
pub fn platformer_white_layer() ->               CollisionLayers {CollisionLayers::from_bits(0b0010000, 0b0010000)}
// pub fn platformer_pickup_weapon_layers() ->     CollisionLayers {CollisionLayers::from_bits(0b001000000, 0b001000000)}
// pub fn platformer_weapon_layers() ->            CollisionLayers {CollisionLayers::from_bits(0b000000000, 0b000000000)}
// pub fn platformer_projectile_damager_layer() -> CollisionLayers {CollisionLayers::from_bits(0b010000001, 0b010000001)} 
// pub fn platformer_projectile_player_layer() ->  CollisionLayers {CollisionLayers::from_bits(0b100000001, 0b100000001)} 
// pub fn platformer_seeker_shapecast_layer() ->   CollisionLayers {CollisionLayers::from_bits(0b000000101, 0b000000101)} 

pub fn player_color_yellow() -> Color {Color::srgba_u8(255, 185, 0, 255)}
pub fn player_color_white() -> Color {Color::srgba_u8(255, 255, 255, 255)}

#[derive(Component)]
pub struct PlatformerEnemy;

#[derive(Component)]
pub struct Toster {
    curr_is_left: bool,
    delay: f32,
    is_dead: bool,
}

#[derive(Component, PartialEq)]
pub enum TosterCaster {
    Left,
    Right,
    Top,
}

#[derive(Component)]
pub struct Cactus;

#[derive(Component)]
pub struct Frog {
    current_dir: FrogCastDir,
    delay: f32,
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Debug)]
pub enum FrogCastDir {
    Right,
    Left,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl FrogCastDir {
    pub fn inverse(&self) -> Self {
        match self {
            &FrogCastDir::Bottom => {
                FrogCastDir::Top
            }
            &FrogCastDir::Top => {
                FrogCastDir::Bottom
            }
            &FrogCastDir::Left => {
                FrogCastDir::Right
            }
            &FrogCastDir::Right => {
                FrogCastDir::Left
            }
            _ => {panic!("ONLY REVERSE STRAIGHT DIR")}
        }
    }
    pub fn handle_orange_case(&self, current_dir: FrogCastDir) -> Self {
        let mut mask;
        match self {
            FrogCastDir::TopRight => {
                mask = Vec2::new(1., 1.)
            }
            FrogCastDir::TopLeft => {
                mask = Vec2::new(-1., 1.)
            }
            FrogCastDir::BottomRight => {
                mask = Vec2::new(1., -1.)
            }
            FrogCastDir::BottomLeft => {
                mask = Vec2::new(-1., -1.)
            }
            _ => {panic!("ONLY REVERSE DIAGONAL DIR")}
        }

        let inverted = current_dir.clone();
        match inverted {
            FrogCastDir::Bottom => {
                mask += Vec2::NEG_Y
            }
            FrogCastDir::Top => {
                mask += Vec2::Y
            }
            FrogCastDir::Left => {
                mask += Vec2::NEG_X
            }
            FrogCastDir::Right => {
                mask += Vec2::X
            }
            _ => {panic!("ONLY REVERSE STRAIGHT DIR (IMPOSSIBLE HERE)")}
        }

        match mask {
            Vec2::X => {
                FrogCastDir::Right
            }
            Vec2::NEG_X => {
                FrogCastDir::Left
            }
            Vec2::Y => {
                FrogCastDir::Top
            }
            Vec2::NEG_Y => {
                FrogCastDir::Bottom
            }
            _ => {current_dir.clone()}
        }
    }
}

const NORMAL_CAST_DISTANCE: f32 = 12.;
const DIAGONAL_CAST_DISTANCE: f32 = 12.;
const CASTER_SCALE: Vec2 = Vec2::splat(0.95);

fn spawn_enemies(
    // enemy: On<Add, PlatformerEnemy>,
    mut cmd: Commands,
    assets: Res<PlatformerAssets>,
) {
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Cactus"),
        Cactus,
        Sprite {
            image: assets.cactus.clone(),
            color: player_color_yellow(),
            ..default()
        },
        // Transform::from_translation(Vec3::new(1060., 1316.0, 0.)),
        RigidBody::Static,
        Collider::capsule(10., 20.), // todo: REMOVE
        platformer_enemy_layer(),
    ));

    let mut collider = Collider::rectangle(60., 40.);
    collider.set_scale(CASTER_SCALE, 10);
    let caster_layers = platformer_raycast_layer().filters;

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Toster"),
        Toster {curr_is_left: true, delay: 0., is_dead: false},
        Sprite {
            image: assets.toster.clone(),
            texture_atlas: Some(TextureAtlas{
                layout: assets.toster_layout.clone(),
                index: 0,
            }),
            color: player_color_yellow(),
            ..default()
        },
        Transform::from_translation(Vec3::new(950., 1316.0, 0.)),
        Collider::rectangle(60., 40.),
        RigidBody::Dynamic,
        platformer_enemy_layer(),
        children![
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::X)
                    .with_max_distance(NORMAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                    TosterCaster::Right,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::NEG_X)
                    .with_max_distance(NORMAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                    TosterCaster::Left,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::Y)
                    .with_max_distance(NORMAL_CAST_DISTANCE)
                    .with_ignore_self(true),
                    TosterCaster::Top,
            ),
        ],
    ));

    let mut collider = Collider::circle(26.);
    collider.set_scale(CASTER_SCALE, 10);
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Frog"),
        Frog {current_dir: FrogCastDir::Left, delay: 0.},
        Sprite {
            image: assets.frog.clone(),
            texture_atlas: Some(TextureAtlas{
                layout: assets.frog_layout.clone(),
                index: 0,
            }),
            color: player_color_yellow(),
            ..default()
        },
        Transform::from_translation(Vec3::new(835.82, 1398.0, 0.)),
        Collider::circle(52. / 2.), // todo: REMOVE
        RigidBody::Dynamic,
        platformer_enemy_layer(),
        LinearVelocity(Vec2::ZERO),
        LockedAxes::ROTATION_LOCKED,
        children![
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::X)
                    .with_max_distance(NORMAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::Right,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::NEG_X)
                    .with_max_distance(NORMAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::Left,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::Y)
                    .with_max_distance(NORMAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::Top,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::NEG_Y)
                    .with_max_distance(NORMAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::Bottom,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::new(Vec2::new(1., 1.,)).unwrap())
                    .with_max_distance(DIAGONAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::TopRight,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::new(Vec2::new(-1., 1.,)).unwrap())
                    .with_max_distance(DIAGONAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::TopLeft,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::new(Vec2::new(1., -1.,)).unwrap())
                    .with_max_distance(DIAGONAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::BottomRight,
            ),
            (
                ShapeCaster::new(collider.clone(), Vec2::ZERO, 0., Dir2::new(Vec2::new(-1., -1.,)).unwrap())
                    .with_max_distance(DIAGONAL_CAST_DISTANCE)
                    .with_ignore_self(true)
                    .with_query_filter(SpatialQueryFilter::from_mask(caster_layers)),
                FrogCastDir::BottomLeft,
            ),
        ]
    ));
}

fn frog_movement(
    enemies_q: Query<(&mut Frog, Entity, &Children)>,
    raycast_q: Query<(&ShapeHits, &FrogCastDir)>,
    mut linvel: Query<&mut LinearVelocity, With<Frog>>,
    player_entity: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_entity) = player_entity.single() {
        for (mut frog, frog_entity, children) in enemies_q {
            let mut raw_hits = HashSet::new();
            for caster_entity in children {
                if let Ok((hits, cast_dir)) = raycast_q.get(*caster_entity) {
                    for hit in hits {
                        if hit.entity != frog_entity && hit.entity != player_entity {
                            raw_hits.insert(cast_dir.clone()); // potential error if collision masks are bad; todo: add collision masks for casters
                        }
                    }
                }
            }
            let final_dir;
            if raw_hits.contains(&FrogCastDir::Left) || raw_hits.contains(&FrogCastDir::Right)
            || raw_hits.contains(&FrogCastDir::Bottom) || raw_hits.contains(&FrogCastDir::Top) {
                raw_hits.remove(&FrogCastDir::BottomLeft);
                raw_hits.remove(&FrogCastDir::BottomRight);
                raw_hits.remove(&FrogCastDir::TopLeft);
                raw_hits.remove(&FrogCastDir::TopRight);
                if raw_hits.len() == 1 {
                    // std case
                    let hit_dir = raw_hits.into_iter().collect::<Vec<FrogCastDir>>()[0].clone();
                    match hit_dir {
                        FrogCastDir::Bottom => {
                            final_dir = FrogCastDir::Left;
                        }
                        FrogCastDir::Top => {
                            final_dir = FrogCastDir::Right;
                        }
                        FrogCastDir::Right => {
                            final_dir = FrogCastDir::Bottom;
                        }
                        FrogCastDir::Left => {
                            final_dir = FrogCastDir::Top;
                        }
                        _ => {unreachable!()}
                    }
                } else if raw_hits.len() == 2 {
                    // pink case
                    if !raw_hits.remove(&frog.current_dir) {
                        warn!("WHOOPS");
                        return;
                    }
                    final_dir = raw_hits.into_iter().collect::<Vec<FrogCastDir>>()[0].inverse();
                } else {
                    warn!("UNWANTED COLLISIONS");
                    return;
                }
            } else if raw_hits.len() != 0 {
                // orange casee
                println!("ABOBOAB {:?}", raw_hits);
                assert!(raw_hits.len() == 1, "MULTIPLE DIAGONAL HITS ON ORANGE");
                let hit_diag_dir = raw_hits.into_iter().collect::<Vec<FrogCastDir>>()[0].clone();
                final_dir = hit_diag_dir.handle_orange_case(frog.current_dir.clone());
                // final_dir = FrogCastDir::Left;
            } else {
                final_dir = frog.current_dir.clone();
            }
            frog.current_dir = final_dir.clone();
            println!("FIN {:?}", final_dir);
            const MS: f32 = 6000.;
            let dir;
            match final_dir {
                FrogCastDir::Bottom => {
                    dir = Vec2::NEG_Y
                }
                FrogCastDir::Top => {
                    dir = Vec2::Y
                }
                FrogCastDir::Left => {
                    dir = Vec2::NEG_X
                }
                FrogCastDir::Right => {
                    dir = Vec2::X
                }
                _ => {unreachable!()}
            }
            let mut lin_vel = linvel.get_mut(frog_entity).unwrap();
            *lin_vel = LinearVelocity::from(dir * MS * time.dt());
        }
    }
}

fn frog_atlas_handler(
    frog_sprite: Query<(&mut Sprite, &mut Frog)>,
    time: Res<Time>,
) {
    for (mut frog_sprite, mut frog) in frog_sprite {
        if frog.delay > 0.2 {
            frog.delay = 0.;
            let atlas = frog_sprite.texture_atlas.as_mut().unwrap();
            atlas.index = (atlas.index + 1) % 8;
        } else {
            frog.delay += time.delta_secs();
        }
    }
}

fn toster_atlas_handler(
    mut cmd: Commands,
    toster_sprite: Query<(&mut Sprite, &mut Toster, Entity)>,
    time: Res<Time>,
) {
    for (mut toster_sprite, mut toster, toster_entity) in toster_sprite {
        if toster.is_dead {
            cmd.entity(toster_entity).remove::<Toster>().insert(RigidBody::Static).despawn_children();
            let atlas = toster_sprite.texture_atlas.as_mut().unwrap();
            atlas.index = 3;
        } else {
            if toster.delay > 0.2 {
                toster.delay = 0.;
                let atlas = toster_sprite.texture_atlas.as_mut().unwrap();
                atlas.index = (atlas.index + 1) % 2;
                if toster.curr_is_left {
                    toster_sprite.flip_x = false;
                } else {
                    toster_sprite.flip_x = true;
                }
            } else {
                toster.delay += time.delta_secs();
            }
        }
    }
}

fn toster_movement(
    toster_q: Query<(&mut Toster, Entity, &Children)>,
    raycast_q: Query<(&ShapeHits, &TosterCaster)>,
    mut linvel: Query<&mut LinearVelocity, With<Toster>>,
    player_entity: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_entity) = player_entity.single() {
        for (mut toster, toster_entity, children) in toster_q {
            for caster_entity in children {
                if let Ok((hits, cast_dir)) = raycast_q.get(*caster_entity) {
                    let mut did_hit = false;
                    for hit in hits {
                        if hit.entity != toster_entity && hit.entity != player_entity {
                            did_hit = true;
                        }
                    }
                    if did_hit {
                        if *cast_dir == TosterCaster::Left {
                            if toster.curr_is_left {
                                toster.curr_is_left = false;
                            }
                        } else {
                            if !toster.curr_is_left {
                                toster.curr_is_left = true
                            }
                        }
                    }
                }
            }
            const MS: f32 = 6000.;
            let mut lin_vel = linvel.get_mut(toster_entity).unwrap();
            let mut dir = Vec2::X;
            if toster.curr_is_left {
                dir = Vec2::NEG_X;
            }
            *lin_vel = LinearVelocity::from(dir * MS * time.dt());
        }
    }
}

fn handle_enemy_collisions(
    mut cmd: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    mut toster_q: Query<&mut Toster>,
    cactus_q: Query<&Cactus>,
    frog_q: Query<&Frog>,
    mut screenshot: ResMut<LastScreenshot>,
    raycast_q: Query<(&ShapeHits, &TosterCaster)>,
    player_entity: Query<Entity, With<Player>>,
    // assets: Res<GeometryDashAssets>, // todo: add death sound
) {
    let mut defeat = false;
    'a: for event in collision_reader.read() {
        if let Ok(_cactus) = cactus_q.get(event.collider2) {
            defeat = true;
            break;
        } else if let Ok(_cactus) = cactus_q.get(event.collider1) {
            defeat = true;
            break;
        }

        if let Ok(_frog) = frog_q.get(event.collider2) {
            defeat = true;
            break;
        } else if let Ok(_frog) = frog_q.get(event.collider1) {
            defeat = true;
            break;
        }

        if let Ok(mut toster) = toster_q.get_mut(event.collider2) {
            if let Ok(player_entity) = player_entity.single() {
                for (hits, cast_dir) in raycast_q {
                    if *cast_dir == TosterCaster::Top {
                        let mut hit_player = false;
                        for hit in hits {
                            if hit.entity == player_entity {
                                hit_player = true;
                                break;
                            }
                        }
                        if hit_player {
                            toster.is_dead = true;
                        } else {
                            defeat = true;
                            break 'a;
                        }
                    }
                }
            }
        } else if let Ok(mut toster) = toster_q.get_mut(event.collider1) {
            if let Ok(player_entity) = player_entity.single() {
                for (hits, cast_dir) in raycast_q {
                    if *cast_dir == TosterCaster::Top {
                        let mut hit_player = false;
                        for hit in hits {
                            if hit.entity == player_entity {
                                hit_player = true;
                                break;
                            }
                        }
                        if hit_player {
                            toster.is_dead = true;
                        } else {
                            defeat = true;
                            break 'a;
                        }
                    }
                }
            }
        }
    }
    if defeat {
        // cmd.spawn((
        //     DespawnOnEnter(NEXT_STATE),
        //     AudioPlayer(assets.explosion.clone()),
        //     PlaybackSettings {
        //         mode: bevy::audio::PlaybackMode::Once,
        //         ..default()
        //     },
        // ));
        if screenshot.awaiting == false {
            cmd.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(await_screenshot_and_translate(AppState::Defeat));
            screenshot.awaiting = true;
        }
    }
}