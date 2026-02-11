use bevy_asset_loader::asset_collection::AssetCollection;
use camera::{CameraController, tick_camera};
use room::Focusable;

use crate::{dev_games::miami::{map::{TilemapShadow, setup_tilemap_shadows}, weapon::{MiamiWeaponSpawner, on_pickup_weapon_collision, on_thrown_weapon_collision, on_weapon_spawnpoint, shoot, throw_weapon, tick_thrown}}, prelude::*};
use super::entity::*;
use crate::miami::shadows::*;
use crate::miami::player::*;

pub const STATE: AppState = AppState::Miami;
pub const NEXT_STATE: AppState = AppState::PacmanEnter;


#[derive(AssetCollection, Resource)]
pub struct MiamiAssets {
    #[asset(path = "maps/miami/map.tmx")]
    pub map: Handle<TiledMapAsset>,
    #[asset(path = "maps/miami/pacman.png")]
    pub character: Handle<Image>,
    #[asset(path = "maps/miami/weapons.png")]
    pub weapons: Handle<Image>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ShadowSystems {
    Update,
}

pub struct MiamiPlugin;

impl Plugin for MiamiPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(
    PhysicsSchedule,
    ShadowSystems::Update
        .after(PhysicsSystems::Last)
        // .after(Systems``::SpatialQuery)
        .after(NarrowPhaseSystems::Last)
        .after(SolverSystems::Finalize)
        .after(avian2d::collision::narrow_phase::CollisionEventSystems)
)
            .register_type::<TilemapShadow>()
            .register_type::<MiamiWeaponSpawner>()
            .register_type::<MiamiEntitySpawner>()

            .add_observer(setup_tilemap_shadows)
            .add_observer(on_weapon_spawnpoint)
            .add_observer(on_entity_spawnpoint)
            .add_observer(on_thrown_weapon_collision)
            .add_observer(on_pickup_weapon_collision)
            

            .add_systems(OnEnter(STATE), setup)
            .add_systems(PreUpdate, (
                (cleanup_shadows, setup_shadows).chain(),
                player_look_at_cursor,
                control_player,
                update_controllers,
                (shoot, throw_weapon).chain(),
                tick_thrown,
                
                // update_shadows,
            ).run_if(in_state(STATE)))
            // .add_systems(PhysicsSchedule, update_shadows.after(tick_camera).in_set(PhysicsSystems::First))
            // .add_systems(PhysicsSchedule, update_shadows.after(tick_camera).in_set(NarrowPhaseSystems::Last))
            .add_systems(PostUpdate, update_shadows.run_if(in_state(STATE)))
            // .add_systems(
            //     PhysicsSchedule,
            // update_shadows.in_set(ShadowSystems::Update).run_if(in_state(STATE))
            // )

            // .add_systems(PostUpdate, (
            //     update_shadows,
            // ).after(TransformSystems::Propagate).run_if(in_state(STATE)))
            .add_systems(Update, (
                tick,
            ).run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

fn setup(
    mut cmd: Commands,
    assets: Res<MiamiAssets>,
    mut camera_controller: ResMut<CameraController>,
){
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Map"),
        TiledMap(assets.map.clone()),
    ))
        ;
    camera_controller.follow_speed = 0.9;
}


fn tick(){}

fn cleanup(
    mut camera: ResMut<CameraController>,
){
    camera.follow_speed = 0.0;
}


pub fn miami_player_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b1000100, 0b1001111)
}
pub fn miami_character_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b0000010, 0b0000111)
}
pub fn miami_dropped_weapon_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b0001000, 0b0000011)
}
pub fn miami_pickup_weapon_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b1000000, 0b1000000)
}
pub fn miami_weapon_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b0000000, 0b0000000)
}
