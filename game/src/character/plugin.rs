use crate::{character::controller::ControllerPlugin, prelude::*};


pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ControllerPlugin)
            // .add_systems(Update, spawn_player_spawnpoint.run_if(run_once))
            ;
    }
}
