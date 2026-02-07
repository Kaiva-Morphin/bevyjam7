use avian2d::prelude::*;
use bevy::prelude::*;

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


#[derive(Component)]
pub struct Player;

pub const MAX_DT : f32 = 0.1;