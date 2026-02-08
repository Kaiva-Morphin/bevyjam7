use avian2d::prelude::*;
use bevy::{camera::visibility::RenderLayers, prelude::*};

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Room, 
}

pub fn player_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Player,
        [ GameLayer::Default, GameLayer::Room ],
    )
}

pub fn room_layers() -> CollisionLayers {
    CollisionLayers::new(
        GameLayer::Room,
        GameLayer::Player
    )
}


pub const MAX_DT : f32 = 0.1;

#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct HighresCamera;

pub const WORLD_LAYERS: RenderLayers = RenderLayers::layer(0);
pub const HIGHRES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub const TARGET_WIDTH: u32 = 576;
pub const TARGET_HEIGHT: u32 = 324;