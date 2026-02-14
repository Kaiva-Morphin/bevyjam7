use crate::{properties::{AppState, LastState}, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;

const STATE: AppState = AppState::PacmanEnter;
const NEXT_STATE: AppState = AppState::End;

pub struct PacmanEatPlugin;

impl Plugin for PacmanEatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick.run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct PacmanEatAssets {
    #[asset(path = "images/splash.png")]
    splash: Handle<Image>,
    #[asset(path = "images/pacman.png")]
    pacman: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 1, rows = 6))]
    pacman_layout: Handle<TextureAtlasLayout>,
}


fn setup(
    mut cmd: Commands,
    assets: Res<PacmanEatAssets>,
    mut state: ResMut<LastState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    state.state = STATE;
    
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Label"),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(vec3(PACMAN_EAT_SCALE, PACMAN_EAT_SCALE, 0.0)),
        Sprite {
            image: assets.splash.clone(),
            ..default()
        },
    ));
    let bg = meshes.add(Rectangle::new(700.0, 700.0));

    let r = meshes.add(Rectangle::new(PACMAN_EAT_PATH_HALF * 2.0, 32.0));
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Background"),
        Mesh2d(bg),
        MeshMaterial2d(materials.add(Color::BLACK)),
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Pacman"),
        Transform::from_translation(Vec3::new(-PACMAN_EAT_PATH_HALF, 0.0, 0.6)).with_scale(vec3(PACMAN_EAT_SCALE, PACMAN_EAT_SCALE, 0.0)),
        Sprite {
            image: assets.pacman.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.pacman_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Pacman,
        children![(
            Mesh2d(r),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_translation(Vec3::new(-PACMAN_EAT_PATH_HALF - 1., 0.0, 0.0))
        )]
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
    let dt = time.dt();
    *t += dt;
    for (mut sprite, mut transform) in q.iter_mut() {
        if *t >= PACMAN_EAT_ANIM_DELAY && let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % 3;
                *t = 0.0;
        }
        transform.translation.x += PACMAN_EAT_WALK_SPEED * dt;
        if transform.translation.x > PACMAN_EAT_PATH_HALF {
            next_state.set(NEXT_STATE);
        }
    }
}

fn cleanup() {}
