use std::f32::consts::PI;

use crate::pathfinder::plugin::*;

use super::plugin::STATE;
use super::entity::*;
use super::player::*;
use super::dialog::*;
use super::plugin::*;
use crate::prelude::*;


#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TilemapShadow;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct BossEntrypointCollider;


#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct EntrypointDialog;


#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct BossDialog;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct HorizontalDoor;


#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct VerticalDoor;


pub fn setup_tilemap_shadows(
    layer_created: On<TiledEvent<LayerCreated>>,
    mut tile_shadow: Query<&mut Transform, With<TilemapShadow>>,
    state: Res<State<AppState>>,
){
    if state.get() != &super::plugin::STATE {return;}
    let e = layer_created.origin;
    let Ok(mut t) = tile_shadow.get_mut(e) else {return;};
    t.translation.x += MIAMI_SHADOW_OFFSET.x;
    t.translation.y += MIAMI_SHADOW_OFFSET.y;
}


#[derive(Event)]
pub struct ObstacleCreated;
// pub fn propagate_obstacles(
//     collider_created: On<TiledEvent<ColliderCreated>>,
//     mut commands: Commands,
//     q: Query<&Children, With<PathfinderObstacle>>,
//     state: Res<State<AppState>>,
//     obstacle: Option<ResMut<SinceObstacle>>,
// ){
//     if state.get() != &super::plugin::STATE {return;}
//     let e = collider_created.origin;

//     for c in q {
//         for c in c {
//             if c == &e {
//                 commands.entity(e).insert(PathfinderObstacle);
//                 if let Some(mut obstacle) = obstacle {
//                     obstacle.0 = 0.0;
//                 }
//                 return;
//             }
//         }
//     }
// }

pub fn propagate_obstacles2(
    mut collider_created: MessageReader<TiledEvent<ColliderCreated>>,
    mut cmd: Commands,
    q: Query<&Children, With<PathfinderObstacle>>,
    state: Res<State<AppState>>,
){
    if state.get() != &super::plugin::STATE {return;}
    let mut i = 0;
    for e in collider_created.read() {
        let e = e.origin;
        i += 1;
        for c in q {
            for c in c {
                if c == &e {
                    cmd.entity(e).insert(PathfinderObstacle);
                }
            }
        }
    }
    if i != 0 {
        cmd.trigger(ObstacleCreated);
    }
}




pub fn obstacle_watcher(
    _ev: On<ObstacleCreated>,
    mut cmd: Commands,
    state: Res<State<AppState>>,
){
    if state.get() != &STATE {return;};
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Navmesh"),
        NavMeshSettings {
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&[
                vec2(0.0, 0.0),
                vec2(1700.0, 0.0),
                vec2(1700.0, 1700.0),
                vec2(0.0, 1700.0),
            ]),
            agent_radius: 6.5,
            simplify: 10.0,
            merge_steps: 3,
            ..default()
        },
        NavMeshUpdateMode::OnDemand(true),
    ));
}

pub fn on_v_door(
    ev: On<Add, VerticalDoor>,
    t: Query<&Transform, With<VerticalDoor>>,
    mut cmd: Commands,
    state: Res<State<AppState>>,
    assets: Res<MiamiAssets>
) {
    if state.get() != &STATE {return};
    let Ok(t) = t.get(ev.entity) else {return;};
    let origin =cmd.spawn((
        Transform::from_translation(t.translation),
        DespawnOnExit(STATE),
        RigidBody::Static,
    )).id();
    let door = cmd.spawn((
        Name::new("Door"),
        Transform::from_translation(t.translation - vec3(0., 16., 0.0)),
        DespawnOnExit(STATE),
        RigidBody::Dynamic,
        LinearDamping(1.0),
        AngularDamping(1.0),
        GravityScale(0.0),
        Mass(100.0),
        Visibility::Inherited,
        CollisionLayers::from_bits(0b101010111, 0b000010110),
        Collider::capsule(4.0, 32.0),
        children![(
            Sprite {
                image: assets.door.clone(),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_z(-PI / 2.)),
        )]
    )).id();
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("DoorJoint"),
        RevoluteJoint::new(origin, door)
        .with_anchor(t.translation.truncate())
        .with_angle_limits(-2., 2.)
    ));
}

pub fn on_h_door(
    ev: On<Add, HorizontalDoor>,
    t: Query<&Transform, With<HorizontalDoor>>,
    mut cmd: Commands,
    state: Res<State<AppState>>,
    assets: Res<MiamiAssets>
) {
    if state.get() != &STATE {return};
    let Ok(t) = t.get(ev.entity) else {return;};
    let origin =cmd.spawn((
        Transform::from_translation(t.translation),
        DespawnOnExit(STATE),
        RigidBody::Static,
    )).id();
    let door = cmd.spawn((
        Name::new("Door"),
        Transform::from_translation(t.translation - vec3(-16., 0., 0.0)).with_rotation(Quat::from_rotation_z(PI / 2.)),
        DespawnOnExit(STATE),
        RigidBody::Dynamic,
        LinearDamping(1.0),
        AngularDamping(1.0),
        GravityScale(0.0),
        Mass(100.0),
        CollisionLayers::from_bits(0b101010111, 0b000010110),
        Collider::capsule(4.0, 32.0),
        Visibility::Inherited,
        children![(
            Sprite {
                image: assets.door.clone(),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_z(-PI / 2.)),
        )]
    )).id();
    cmd.spawn((
        Name::new("DoorJoint"),
        RevoluteJoint::new(origin, door)
        .with_local_frame2(Isometry2d::from_rotation(Rot2::from_sin_cos(-1., 0.)))
        .with_anchor(t.translation.truncate())
        .with_angle_limits(-3., 3.)
    ));
}


pub fn on_entrypoint_dialog_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    spawn_query: Query<&EntrypointDialog>,
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
        Name::new("EntrypointDialog"),
        Sensor,
        EntrypointDialog,
        dialog_sensor_layer(),
        RigidBody::Static,
        CollisionEventsEnabled,
    )).observe(on_entrypoint_dialog_collision);
}

pub fn on_boss_entrypoint_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    spawn_query: Query<&BossEntrypointCollider>,
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
        Name::new("BossEntrypointCollider"),
        Sensor,
        dialog_sensor_layer(),
        BossEntrypointCollider,
        RigidBody::Static,
        CollisionEventsEnabled,
    ));
}

pub fn on_boss_dialog_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    spawn_query: Query<&BossDialog>,
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
        dialog_sensor_layer(),
        Name::new("BossEntrypointCollider"),
        Sensor,
        BossDialog,
        RigidBody::Static,
        CollisionEventsEnabled,
    )).observe(on_boss_dialog_collision);
}

pub fn on_entrypoint_dialog_collision(
    e: On<CollisionStart>,
    q: Query<&EntrypointDialog>,
    mut cmd: Commands,
    state: Res<State<AppState>>,
    assets: Res<MiamiAssets>,
    cam: Query<Entity, With<WorldCamera>>,
    pq: Query<(Entity, &mut CharacterController), (With<Player>, Without<PlayerDisabled>)>,
    mut shooted: ResMut<ShootedDialogs>,
) {
    if state.get() != &STATE {return;}
    if shooted.entrypoint {return;}
    shooted.entrypoint = true;

    let spawn_entity = e.event().collider1;
    let Ok(_p) = q.get(spawn_entity) else {return;};
    let Some(cam) = cam.iter().next() else {return;};

    for (p, mut c) in pq {
        cmd.entity(p).insert(PlayerDisabled);
        c.input_dir = Vec2::ZERO;
        c.throw = false;
        c.shoot = false;
    }
    
    start_entrypoint_dialog(&mut cmd, &assets, cam);
}

pub fn on_boss_dialog_collision(
    e: On<CollisionStart>,
    q: Query<&BossDialog>,
    mut cmd: Commands,
    state: Res<State<AppState>>,
    assets: Res<MiamiAssets>,
    cam: Query<Entity, With<WorldCamera>>,
    pq: Query<(Entity, &mut CharacterController), (With<Player>, Without<PlayerDisabled>)>,
    mut shooted: ResMut<ShootedDialogs>,
) {
    if state.get() != &STATE {return;}
    if shooted.boss {return;}
    shooted.boss = true;

    let spawn_entity = e.event().collider1;
    let Ok(_p) = q.get(spawn_entity) else {return;};
    let Some(cam) = cam.iter().next() else {return;};

    for (p, mut c) in pq {
        cmd.entity(p).insert(PlayerDisabled);
        c.input_dir = Vec2::ZERO;
        c.throw = false;
        c.shoot = false;
    }
    
    start_boss_dialog(&mut cmd, &assets, cam);
}

pub fn block_bossroom(cmd: &mut Commands, q: &Query<Entity, With<BossEntrypointCollider>>) {
    for e in q.iter() {
        cmd.entity(e).insert((
            DespawnOnExit(STATE),
            Name::new("BossEntrypointCollider"),
            dialog_sensor_layer(),
            BossEntrypointCollider,
            RigidBody::Static,
            CollisionEventsEnabled,
        )).remove::<Sensor>();
    }
}
