use crate::{properties::{AppState, LastScreenshot, LastState}, prelude::*};
use avian2d::math::Vector;
use bevy_asset_loader::asset_collection::AssetCollection;
use room::{Focusable, RoomController, on_room_spawned};
use camera::CameraController;

const STATE: AppState = AppState::Platformer;
const NEXT_STATE: AppState = AppState::Miami;

pub struct PlatformerPlugin;

impl Plugin for PlatformerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(STATE), (
                setup,
            ))
            .add_systems(Update, tick.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup)
            .register_type::<NextTrigger>()
            .register_type::<StopTrigger>()
            .add_observer(focus_player)
            .add_observer(on_collision)
            .add_observer(on_room_spawned)
            .add_observer(on_stop_spawned)
            .add_observer(on_next_spawned)
            ;
    }
}

#[derive(Component)]
struct Player;

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
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    character_layout: Handle<TextureAtlasLayout>,
}

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

    let collider = Collider::rectangle(16.0, 16.0);
    let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * Vector::new(0.99, 1.01), 10);
    
    let player = cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Player"),
        Sprite {
            image: assets.character.clone(),
            texture_atlas: Some(TextureAtlas{
                layout: assets.character_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Player,
        player_layers(),
        RigidBody::Dynamic,
        LockedAxes::new().lock_rotation(),
        collider,
        CollisionEventsEnabled,
        Focusable,
        GravityScale(PLATFORMER_GRAVITY_FORCE),
        Transform::from_translation(pt),
        ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(8.1)
                .with_ignore_self(true)
                .with_query_filter(SpatialQueryFilter::from_mask(GameLayer::Default)),
        Friction::new(0.0),
    )).id();
    
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
) {
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
    // mut q: Query<(&mut Sprite, &mut Transform), With<Pacman>>,
) {
    let dt = time.dt();
    let mut grounded = false;
    for (_entity, mut linvel, hits, mut sprite) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
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
            *t += dt;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            target += PLATFORMER_MAX_SPEED;
            sprite.flip_x = false;
            *t += dt;
        }

        if grounded && *t < PLATFORMER_ANIM_DELAY * 2.0 && let Some(ta) = &mut sprite.texture_atlas {
            let i = (*t / PLATFORMER_ANIM_DELAY).floor() as usize;
            ta.index = i;
            
        }
        if *t > PLATFORMER_ANIM_DELAY * 2.0 {
            *t = 0.0;
        }
        if !grounded && let Some(ta) = &mut sprite.texture_atlas {
            if linvel.y > 0.0 {
                ta.index = 2;
            } else {
                ta.index = 3;
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
    e_q: Query<&GlobalTransform,With<StopTrigger>>,
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
            // cmd.spawn(bevy::render::view::screenshot::Screenshot::image(canvas.image.clone()))
            cmd.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(await_screenshot_and_translate(NEXT_STATE))
                // .observe(bevy::render::view::screenshot::save_to_disk("./screen.png"))
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
        Name::new("Stop"),
        Sensor,
        StopTrigger,
        RigidBody::Static,
        CollisionEventsEnabled,
    ));
}