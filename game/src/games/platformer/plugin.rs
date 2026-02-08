use crate::{games::plugin::AppState, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;
use room::RoomController;

const STATE: AppState = AppState::Platformer;
const NEXT_STATE: AppState = AppState::PacmanEnter;

pub struct PlatformerPlugin;

impl Plugin for PlatformerPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(RoomController::default())
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

struct NextTrigger;
struct StopTrigger;


#[derive(AssetCollection, Resource)]
pub struct PlatformerAssets {
    #[asset(path = "maps/platformer/map.tmx")]
    tilemap: Handle<TiledMapAsset>,
    #[asset(path = "images/pacman.png")]
    pacman: Handle<Image>,
}


fn setup(
    mut cmd: Commands,
    assets: Res<PlatformerAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Platformer setup");
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Map"),
        TiledMap(assets.tilemap.clone()),
    ));
    cmd.spawn((
        Sprite {
            image: assets.pacman.clone(),
            ..default()
        }
    ));
}

#[derive(Component)]
pub struct Pacman;


fn tick (
    time: Res<Time>,
    mut t: Local<f32>,
    mut next_state: ResMut<NextState<AppState>>,
    mut q: Query<(&mut Sprite, &mut Transform), With<Pacman>>,
) {

}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
) {
    cmd.remove_resource::<RoomController>();
    cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
}
