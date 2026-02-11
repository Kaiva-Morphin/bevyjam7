use bevy_asset_loader::asset_collection::AssetCollection;
use camera::{CameraController, tick_camera};
use room::Focusable;

use crate::{dev_games::miami::{map::{TilemapShadow, propagate_obstacles, setup_tilemap_shadows}, weapon::{MiamiWeaponSpawner, health_watcher, on_pickup_weapon_collision, on_projectile_hit, on_thrown_weapon_collision, on_weapon_spawnpoint, shoot, throw_weapon, tick_thrown, update_projectile}}, prelude::*};
use super::entity::*;
use crate::miami::shadows::*;
use crate::miami::player::*;

pub const STATE: AppState = AppState::Miami;
pub const NEXT_STATE: AppState = AppState::PacmanEnter;


#[derive(AssetCollection, Resource)]
pub struct MiamiAssets {
    #[asset(path = "maps/miami/map.tmx")]
    pub map: Handle<TiledMapAsset>,
    #[asset(path = "maps/miami/weapons.png")]
    pub weapons: Handle<Image>,
    #[asset(path = "maps/miami/pacman.png")]
    pub character: Handle<Image>,
    #[asset(path = "maps/miami/endoskeleton.png")]
    pub endoskeleton: Handle<Image>,
    #[asset(path = "maps/miami/bonnie.png")]
    pub bonnie: Handle<Image>,
    #[asset(path = "maps/miami/decals.png")]
    pub decals: Handle<Image>,
    #[asset(path = "maps/miami/projectiles.png")]
    pub projectiles: Handle<Image>,
}

pub struct MiamiPlugin;

impl Plugin for MiamiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TilemapShadow>()
            .register_type::<MiamiWeaponSpawner>()
            .register_type::<MiamiEntitySpawner>()

            .add_observer(setup_tilemap_shadows)
            .add_observer(on_weapon_spawnpoint)
            .add_observer(on_entity_spawnpoint)
            .add_observer(on_thrown_weapon_collision)
            .add_observer(on_pickup_weapon_collision)
            .add_observer(propagate_obstacles)
            .add_observer(on_projectile_hit)
            

            .add_systems(OnEnter(STATE), (
                setup,
                // setup_navmesh
            ))
            .add_systems(PreUpdate, (
                (cleanup_shadows, setup_shadows).chain(),
                update_projectile,
                player_look_at_cursor,
                update_controllers,
                (control_player, shoot, throw_weapon).chain(),
                tick_thrown,
                update_chasers,
                chase,
                // display_path,
                
                // update_shadows,
            ).run_if(in_state(STATE)))
            // .add_systems(PhysicsSchedule, update_shadows.after(tick_camera).in_set(PhysicsSystems::First))
            // .add_systems(PhysicsSchedule, update_shadows.after(tick_camera).in_set(NarrowPhaseSystems::Last))
            .add_systems(PostUpdate, (update_shadows, health_watcher).run_if(in_state(STATE)))
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
    camera_controller.target_zoom = 0.9;
}


fn tick(){}

fn cleanup(
    mut camera: ResMut<CameraController>,
){
    camera.follow_speed = 0.0;
}

pub fn miami_player_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b11000110, 0b11000111)
}
pub fn miami_character_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b10010010, 0b10010111)
}
pub fn miami_dropped_weapon_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b00011000, 0b00010011)
}
pub fn miami_pickup_weapon_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b01000000, 0b01000000)
}
pub fn miami_weapon_layers() -> CollisionLayers {
    CollisionLayers::from_bits(0b00000000, 0b00000000)
}
pub fn miami_projectile_damager_layer() -> CollisionLayers {
    CollisionLayers::from_bits(0b10000001, 0b10000001)
} 
pub fn miami_seeker_shapecast_layer() -> CollisionLayers {
    CollisionLayers::from_bits(0b00000101, 0b00000101)
} 


pub fn red_blood() -> Color {Color::Srgba(Srgba::rgba_u8(200, 32, 61, 255))}
pub fn oil_blood() -> Color {Color::Srgba(Srgba::rgba_u8(30, 22, 64, 255))}
pub fn blood_rects() -> [Rect; 3] {
    [
        Rect::new(16.0, 32.0, 32.0, 48.0),
        Rect::new(0.0, 32.0, 16.0, 48.0),
        Rect::new(0.0, 0.0, 32.0, 32.0),
    ]
}
pub fn front_body_rect() -> Rect {Rect::new(48.0, 0.0, 80.0, 96.0)}
pub fn back_body_rect() -> Rect {Rect::new(80.0, 0.0, 112.0, 96.0)}

pub const BLOOD_Z_TRANSLATION : f32 = -6.0;
pub const BODY_Z_TRANSLATION : f32 = -4.0;
pub const THROWN_DAMAGE_MULTIPLIER: f32 = 0.0071428571;
