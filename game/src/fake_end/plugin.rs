use std::{f32::consts::PI, time::Duration};

use bevy::{asset::RenderAssetUsages, camera::{RenderTarget, visibility::RenderLayers}, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}};
use bevy_asset_loader::asset_collection::AssetCollection;
use camera::ViewportCanvas;

use crate::{games::plugin::{AppState}, prelude::*};

const STATE: AppState = AppState::FakeEnd;
const NEXT_STATE: AppState = AppState::Platformer;

const RECT_HS: f32 = 3.;
#[derive(Component)]
struct TextureRect;

#[derive(AssetCollection, Resource)]
pub struct FakeEndAssets {
    #[asset(path = "yaroholder.png")]
    text: Handle<Image>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)]
#[states(scoped_entities)]
enum LocalState {
    #[default]
    InitialAnim,
    Game,
    Defeat,
    Win,
    Aboba,
}

pub struct FakeEndPlugin;
impl Plugin for FakeEndPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(LocalRes::default())
            // .insert_resource(Pipes::default())
            .add_sub_state::<LocalState>()
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick_transition.run_if(in_state(LocalState::InitialAnim)))
            // .add_systems(OnEnter(LocalState::Game), begin_game)
            .add_systems(Update, monke_fall.run_if(in_state(LocalState::Game)))
            // .add_systems(Update, tick_defat.run_if(in_state(LocalState::Defeat)))
            // .add_systems(Update, tick_win.run_if(in_state(LocalState::Win)))
            .add_systems(OnEnter(LocalState::Aboba), cleanup)
            // .add_observer(collision_handler)
            ;
    }
}

fn tick_transition(
    mut state: ResMut<NextState<LocalState>>,
) {
    state.set(LocalState::Game);
}

fn setup(
    mut cmd: Commands,
    assets: Res<FakeEndAssets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cam: Query<Entity, With<WorldCamera>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    
    let image_handle = images.add(image);

    // cmd.spawn((
    //     DespawnOnExit(STATE),
    //     Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //     Sprite {
    //         image: image_handle.clone(),
    //         ..default()
    //     },
    // ));
    let cam = cam.iter().next().expect("No cam!");
    let screen = cmd.spawn((
            UiTargetCamera(cam),
            Node{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode {
                image: image_handle.clone(),
                ..default()
            },
    ZIndex(100)));

    cmd
        .spawn((
            DespawnOnExit(STATE),
            Camera3d::default(),
            // Camera2d,
            Camera {
                clear_color: ClearColorConfig::Custom(Color::Srgba(Srgba::rgba_u8(0, 0, 0, 0))),
                order: -2,
                ..default()
            },
            RenderTarget::Image(image_handle.clone().into()),
            RenderLayers::layer(3),
            Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

    // Lighting
    cmd.spawn((
        DespawnOnExit(STATE),
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    let mesh = meshes.add(Mesh::from(Rectangle {
        half_size: Vec2::splat(RECT_HS),
    }));
    
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.text.clone()),
        ..default()
    });
    
    cmd.spawn((
        Name::new("Texture rect"),
        DespawnOnExit(STATE),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        RenderLayers::layer(3),
        TextureRect,
    ));

    cmd.insert_resource(FallStart {start: false, num: 0, timer: 0.});
}

#[derive(Resource)]
pub struct FallStart {
    pub start: bool,
    pub num: usize,
    pub timer: f32,
}

fn monke_fall(
    mut transform_q: Query<&mut Transform, With<TextureRect>>,
    mut fall_start: ResMut<FallStart>,
    mut state: ResMut<NextState<LocalState>>,
    time: Res<Time>,
) {
    // fall_start.timer += time.delta_secs();
    // if fall_start.timer >= 0.0 {
    //     fall_start.start = true;
    // }
    const DEGPF: f32 = -0.01;
        if fall_start.num as f32 * -DEGPF > PI / 2. {
            state.set(LocalState::Aboba);
        }
        let pivot_point = Vec3::new(0.0, -RECT_HS, 0.0);
        let q = Quat::from_axis_angle(Vec3::X, DEGPF);
        transform_q.single_mut().expect("no rect").rotate_around(pivot_point, q);
        fall_start.num += 1;
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
) {
    cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
    cmd.remove_resource::<FallStart>();
}