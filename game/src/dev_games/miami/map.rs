use super::plugin::STATE;
use crate::{pathfinder::plugin::PathfinderObstacle, prelude::*};

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

pub fn propagate_obstacles(
    collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
    q: Query<&Children, With<PathfinderObstacle>>,
    state: Res<State<AppState>>,
){
    if state.get() != &super::plugin::STATE {return;}
    let e = collider_created.origin;
    for c in q {
        for c in c {
            if c == &e {
                commands.entity(e).insert(PathfinderObstacle);
                commands.spawn((
                    DespawnOnExit(STATE),
                    Name::new("Navmesh"),
                    NavMeshSettings {
                        // Define the outer borders of the navmesh.
                        fixed: Triangulation::from_outer_edges(&[
                            vec2(0.0, 0.0),
                            vec2(1000.0, 0.0),
                            vec2(1000.0, 1000.0),
                            vec2(0.0, 1000.0),
                        ]),
                        agent_radius: 6.0,
                        simplify: 1.0,
                        merge_steps: 1,
                        ..default()
                    },
                    NavMeshUpdateMode::Direct,
                ));
                return;
            }
        }
    }
}


pub fn on_v_door(
    ev: On<Add, VerticalDoor>,
    t: Query<&GlobalTransform, With<VerticalDoor>>,
    mut cmd: Commands
) {
    let Ok(t) = t.get(ev.entity) else {return;};
    // cmd.spawn()
}