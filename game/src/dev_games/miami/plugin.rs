use bevy_asset_loader::asset_collection::AssetCollection;
use camera::{CameraController, tick_camera};
use room::Focusable;

use crate::{dev_games::miami::map::{TilemapShadow, setup_tilemap_shadows}, dev_games::{miami::map::*, plugin::*}, prelude::*};
use super::entity::*;

pub const STATE: AppState = AppState::Miami;
pub const NEXT_STATE: AppState = AppState::PacmanEnter;


#[derive(AssetCollection, Resource)]
pub struct MiamiAssets {
    #[asset(path = "maps/miami/map.tmx")]
    map: Handle<TiledMapAsset>,
    #[asset(path = "maps/miami/pacman.png")]
    character: Handle<Image>,
}


pub struct MiamiPlugin;

impl Plugin for MiamiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TilemapShadow>()

            .add_observer(setup_tilemap_shadows)

            .add_systems(OnEnter(STATE), setup)
            .add_systems(PreUpdate, (
                setup_shadows,
                update_shadows,
                player_look_at_cursor,
                control_player,
                update_controllers,
            ).run_if(in_state(STATE)))
            .add_systems(PhysicsSchedule, 
                update_shadows.in_set(NarrowPhaseSystems::Last)
                .before(tick_camera)
                .run_if(in_state(STATE))
            )
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
    ));

    let char = default_character();
    let player = cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Character"),
        GlobalTransform::default(),
        Visibility::default(),
        Focusable,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::circle(7.0),
        Player,
        GravityScale(0.0),
        CharacterController{input_dir: Vec2::ZERO, speed:100.0, look_dir: Vec2::ZERO},
        children![(
            CharacterPivotPoint,
            Visibility::default(),
            GlobalTransform::default(),
            Transform::default(),
            children![(
                ShadowInit,
                Sprite {
                    image: assets.character.clone(),
                    rect: Some(char.default_rect.clone()),
                    ..Default::default()
                },
                char,
            )],
        )],
    )).id();
    camera_controller.focused_entities.push_front(player);
}


fn tick(){}
fn cleanup(){}
