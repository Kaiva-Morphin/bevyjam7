use crate::{character::controller::CharacterController, prelude::*, tilemap::plugin::SpawnPoint};

// #[derive(Component, Reflect, Default)]
// #[reflect(Component, Default)]
// pub struct PlayerSpawnpoint;

pub fn spawn_player(
    commands: &mut Commands,
    transform: Transform,
) {
    commands.spawn((
        Name::new("Player"),
        transform,
        RigidBody::Dynamic,
        Collider::circle(5.0),
        GravityScale(0.0),
        player_layers(),
        CollisionEventsEnabled,
        CharacterController::default(),
    )); 
}


pub fn spawn_player_spawnpoint(
    mut cmd: Commands,
    spawnpoints: Query<(&Transform, &SpawnPoint)>,
) {
    for (t, sp) in spawnpoints.iter() {
        match sp {
            SpawnPoint::Player => {
                spawn_player(&mut cmd, t.clone());
                return;
            }
            _ => {}
        }
    }
    warn!("No player spawnpoint found");
}