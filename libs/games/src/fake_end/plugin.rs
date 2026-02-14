use std::f32::consts::PI;

use bevy::{asset::RenderAssetUsages, camera::{RenderTarget, visibility::RenderLayers}, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;

const STATE: AppState = AppState::FakeEnd;
const NEXT_STATE: AppState = AppState::Novel;

const RECT_HS: f32 = 3.;
#[derive(Component)]
struct TextureRect;

#[derive(AssetCollection, Resource)]
pub struct FakeEndAssets {
    #[asset(path = "sounds/creak1.mp3")]
    creek1: Handle<AudioSource>,
    #[asset(path = "sounds/creak2.mp3")]
    creek2: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct JokerTexture {
    pub handle: Handle<Image>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)]
#[states(scoped_entities)]
enum LocalState {
    #[default]
    Game,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum SuperLocalState {
    #[default]
    Setup,
    Game,
    JokerUp,
    JokerDown,
}

#[derive(Resource, PartialEq, Eq)]
pub struct JokerSetUp(bool);

pub struct FakeEndPlugin;
impl Plugin for FakeEndPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, global_setup.after(camera::setup_camera)) // .after(crate::||  ||::plugin::setup) todo:!!
            .init_state::<SuperLocalState>()
            .add_sub_state::<LocalState>()
            .insert_resource(JokerSetUp(false))
            .add_systems(OnEnter(STATE), setup)
            // .add_systems(OnEnter(LocalState::Game), begin_game)
            .add_systems(Update, 
                (monke_fall, paste_screenshot).run_if(not(in_state(SuperLocalState::JokerDown))).run_if(resource_equals(JokerSetUp(true))))
            // .add_systems(Update, tick_defat.run_if(in_state(LocalState::Defeat)))
            // .add_systems(Update, tick_win.run_if(in_state(LocalState::Win)))
            .add_systems(Update, cleanup.run_if(in_state(SuperLocalState::JokerDown)))
            // .add_observer(collision_handler)
            ;
    }
}

fn global_setup(
    mut cmd: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cam: Query<Entity, With<HighresCamera>>,
    asset_server: Res<AssetServer>
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
    let cam = cam.iter().next().expect("No cam!");
    cmd.spawn((
        DespawnOnEnter(SuperLocalState::JokerDown),
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
    ZIndex(10000)));

    cmd.spawn((
        DespawnOnEnter(SuperLocalState::JokerDown),
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::Srgba(Srgba::rgba_u8(0, 0, 0, 0))),
            order: -2,
            ..default()
        },
        RenderTarget::Image(image_handle.clone().into()),
        RenderLayers::layer(3),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    let mesh = meshes.add(Mesh::from(Rectangle {
        half_size: Vec2::splat(RECT_HS),
    }));

    let text = asset_server.load("yaroholder.png");
    
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(text.clone()),
        ..default()
    });
    let mut t = Transform::from_translation(Vec3::new(0.0, 0., -2.));
    let pivot_point = Vec3::new(0.0, -RECT_HS, -2.);
    let q = Quat::from_axis_angle(Vec3::X, PI / 2.);
    t.rotate_around(pivot_point, q);
    cmd.spawn((
        Name::new("Texture rect"),
        DespawnOnEnter(SuperLocalState::JokerDown),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        RenderLayers::layer(3),
        TextureRect,
        t,
    ));
    cmd.insert_resource(JokerTexture {handle: image_handle});
}

fn setup(
    mut cmd: Commands,
    mut joker_rect: Query<&mut Transform, With<TextureRect>>,
    mut super_local_state: ResMut<NextState<SuperLocalState>>,
    mut joker_set_up: ResMut<JokerSetUp>,
) {
    joker_set_up.0 = true;
    super_local_state.set(SuperLocalState::Game);
    cmd.insert_resource(FallStart {start: false, num: 0, timer: 0.});
    cmd.insert_resource(ClimbStart {climbed: false, num: 0, timer: 0.});
    let pivot_point = Vec3::new(0.0, -RECT_HS, -2.);
    let q = Quat::from_axis_angle(Vec3::X, PI / 2.);
    let mut t = joker_rect.single_mut().expect("NO JOKER");
    *t = Transform::from_translation(Vec3::new(0.0, 0.0, -2.0));
    t.rotate_around(pivot_point, q);
}

#[derive(Resource)]
pub struct FallStart {
    pub start: bool,
    pub num: usize,
    pub timer: f32,
}

#[derive(Resource)]
pub struct ClimbStart {
    pub num: usize,
    pub timer: f32,
    pub climbed: bool,
}

fn monke_fall(
    mut cmd: Commands,
    mut transform_q: Query<&mut Transform, With<TextureRect>>,
    fall_start: Option<ResMut<FallStart>>,
    climb_start: Option<ResMut<ClimbStart>>,
    time: Res<Time>,
    fake_assets: Option<Res<FakeEndAssets>>,
    mut super_local_state: ResMut<NextState<SuperLocalState>>,
    mut appstate: ResMut<NextState<AppState>>,
) {
    if let Some(mut fall_start) = fall_start {
        let mut climb_start = climb_start.unwrap();
        let fake_assets = fake_assets.unwrap();
        const DEGPF: f32 = -0.01;
        if climb_start.num as f32 * -DEGPF > PI / 2. {
            climb_start.timer += time.delta_secs();
            if climb_start.timer >= 2.0 {
                fall_start.start = true;
            }
        } else {
            if climb_start.num == 0 {
                cmd.spawn((
                    DespawnOnEnter(SuperLocalState::JokerDown),
                    AudioPlayer(fake_assets.creek1.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Once,
                        ..default()
                    },
                ));
            }
            let pivot_point = Vec3::new(0.0, -RECT_HS, -2.);
            let q = Quat::from_axis_angle(Vec3::X, DEGPF);
            transform_q.single_mut().expect("no rect").rotate_around(pivot_point, q);
            climb_start.num += 1;
        }
        if fall_start.start {
            if fall_start.num == 0 {
                appstate.set(NEXT_STATE);
                super_local_state.set(SuperLocalState::JokerUp);
                cmd.spawn((
                    DespawnOnEnter(SuperLocalState::JokerDown),
                    AudioPlayer(fake_assets.creek2.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Once,
                        ..default()
                    },
                ));
            }
            if fall_start.num as f32 * -DEGPF > PI {
                super_local_state.set(SuperLocalState::JokerDown);
            }
            let pivot_point = Vec3::new(0.0, -RECT_HS, -2.);
            let q = Quat::from_axis_angle(Vec3::X, DEGPF);
            transform_q.single_mut().expect("no rect").rotate_around(pivot_point, q);
            fall_start.num += 1;
        }
    }
}

fn paste_screenshot(
    last: Res<LastScreenshot>,
    mut cmd: Commands,
    cam: Query<Entity, With<HighresCamera>>,
    mut ran: Local<bool>
) {
    if !*ran {
        if let Some(img) = last.image.clone() {
            println!("ASDSASDSD");
            *ran = true;
            let cam = cam.iter().next().expect("No cam!");
            cmd.spawn((
                Name::new("Screenshot"),
                DespawnOnEnter(SuperLocalState::JokerUp),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                // RenderLayers::layer(3),
                UiTargetCamera(cam),
                ImageNode {
                    image: img,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ZIndex(1000),
            ));
        }
    }
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
    mut ran: Local<bool>
) {
    if !*ran {
        *ran = true;
        cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
        cmd.remove_resource::<FallStart>();
        cmd.remove_resource::<ClimbStart>();
    }
}