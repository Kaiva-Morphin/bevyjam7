use std::f32::consts::FRAC_2_PI;

use bevy_asset_loader::asset_collection::AssetCollection;
use camera::CameraController;

use super::{map::{TilemapShadow, propagate_obstacles, setup_tilemap_shadows}, weapon::{MiamiWeaponSpawner, health_watcher, on_pickup_weapon_collision, on_projectile_hit, on_thrown_weapon_collision, on_weapon_spawnpoint, shoot, throw_weapon, tick_thrown, update_projectile}};
use crate::prelude::*;
use super::entity::*;
use super::shadows::*;
use super::player::*;
use super::dialog::*;

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

    #[asset(path = "maps/miami/dialog_faz.png")]
    pub dialog_faz: Handle<Image>,
    #[asset(path = "maps/miami/dialog_pac.png")]
    pub dialog_pac: Handle<Image>,

    #[asset(path = "fonts/kaivs_minegram_v1.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "fonts/kaivs_minegram_v1-italic.ttf")]
    pub italic: Handle<Font>,
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
            .add_observer(on_map_created)
            

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
                tick,
                update_chasers,
                chase,
                tick_dialog,
                cleanup_tweens
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
    cam: Query<Entity, With<WorldCamera>>,
){
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Map"),
        TiledMap(assets.map.clone()),
    ))
        ;
    let cam = cam.iter().next().expect("No cam!");

    start_dialog(&mut cmd, &assets, cam, vec![
        ("HELLO, PAC! I WILL KILL YOU!".to_string(), Speaker::Freddy),
        ("YOU BASTARD!".to_string(), Speaker::Pacman)
    ]);
}

fn on_map_created(
    _event: On<TiledEvent<TilemapCreated>>,
    state: Res<State<AppState>>,
    mut map: Query<&mut Transform, With<TiledMap>>,
) {
    if state.get() != &STATE {return;};
    let Ok(mut map) = map.single_mut() else {return;};
    map.scale.z = 0.05;
}


pub fn late_setup(
    mut camera_controller: ResMut<CameraController>,
){
    camera_controller.follow_speed = 0.9;
    camera_controller.target_zoom = 0.9;
}


fn tick(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<WorldCamera>>
){
    let Some(mut t) = camera.iter_mut().next() else {return;};
    t.rotation.z = (time.elapsed_secs() * 0.7).sin() * 0.02;
}

fn cleanup(
    mut controller: ResMut<CameraController>,
    mut camera: Query<&mut Transform, With<WorldCamera>>,
){
    controller.follow_speed = 0.0;
    controller.target_zoom = 0.8;
    let Ok(mut t) = camera.single_mut() else {return;};
    t.rotation.z = 0.0;
    t.rotation.y = 0.0;
}

pub fn miami_player_layers() ->            CollisionLayers {CollisionLayers::from_bits(0b101000110, 0b101000111)}
pub fn miami_character_layers() ->         CollisionLayers {CollisionLayers::from_bits(0b010010010, 0b010010111)}
pub fn miami_dropped_weapon_layers() ->    CollisionLayers {CollisionLayers::from_bits(0b000011000, 0b000010011)}
pub fn miami_pickup_weapon_layers() ->     CollisionLayers {CollisionLayers::from_bits(0b001000000, 0b001000000)}
pub fn miami_weapon_layers() ->            CollisionLayers {CollisionLayers::from_bits(0b000000000, 0b000000000)}
pub fn miami_projectile_damager_layer() -> CollisionLayers {CollisionLayers::from_bits(0b010000001, 0b010000001)} 
pub fn miami_projectile_player_layer() ->  CollisionLayers {CollisionLayers::from_bits(0b100000001, 0b100000001)} 
pub fn miami_seeker_shapecast_layer() ->   CollisionLayers {CollisionLayers::from_bits(0b000000101, 0b000000101)} 


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
