use crate::prelude::*;


#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct PathfinderObstacle;

pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PathfinderObstacle>()
            .add_plugins((
                VleueNavigatorPlugin,
                NavmeshUpdaterPlugin::<Collider, PathfinderObstacle>::default(),
            ))
            .insert_resource(NavMeshesDebug(bevy::color::palettes::tailwind::RED_800.into()))
            ;
    }
}