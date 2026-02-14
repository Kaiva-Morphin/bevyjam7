use std::collections::HashMap;

use bevy::{color::palettes::css::BLACK, log::tracing, prelude::*, window::WindowResized};
use bevy_asset_loader::asset_collection::AssetCollection;
use games::prelude::{AppState, LastScreenshot};
use properties::{HighresCamera, WorldCamera};
use utils::WrappedDelta;

use crate::dev_games::fnaf::plugin::FNAFAssets;

const STATE: AppState = AppState::End;

pub struct FinalPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)]
#[states(scoped_entities)]
enum LocalState {
    #[default]
    Initial,
    Bear,
    Zooming,
    Animation,
    Finished,
}

#[derive(AssetCollection, Resource)]
pub struct FinalAssets {
    #[asset(path = "images/final/black.png")]
    black: Handle<Image>,
    #[asset(path = "images/final/the_question_of_the_bear.png")]
    question: Handle<Image>,
    #[asset(path = "images/final/CJ.png")]
    cj: Handle<Image>,
    #[asset(path = "images/final/clown.png")]
    clown: Handle<Image>,
    #[asset(path = "images/final/miami.png")]
    miami: Handle<Image>,
    #[asset(path = "images/final/cactus.png")]
    cactus: Handle<Image>,
    #[asset(path = "images/final/flappybird.png")]
    flappy_bird: Handle<Image>,
    #[asset(path = "images/final/bavy.png")]
    bavy: Handle<Image>,

    #[asset(path = "sounds/final/131599__echocinematics__kill-switch-large-breaker-switch.mp3")]
    lights_out: Handle<AudioSource>,
    #[asset(path = "sounds/final/261042__johnthewizar__house-organ-f1.wav")]
    freddy_appears: Handle<AudioSource>,
    #[asset(path = "sounds/final/78127__jovica__layers-006-chrunched-church-organ-drone-pad-b7.mp3")]
    zoom_out: Handle<AudioSource>,
    #[asset(path = "sounds/final/566195__scholzi982__press_button_02.wav")]
    button_press: Handle<AudioSource>,
    #[asset(path = "sounds/final/784061__newlocknew__comtv_turning-onoffor-switching-tv-channels.mp3")]
    tv_off: Handle<AudioSource>,
}

impl Plugin for FinalPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_sub_state::<LocalState>()
            .add_systems(OnEnter(STATE), (spawn_texture_holder, setup, debug_cameras).chain())
            .add_systems(Update, (timing_system, resize_texture_holder).run_if(in_state(STATE)))
            .add_systems(Update, start_zoom.run_if(in_state(LocalState::Bear)))
            .add_systems(Update, (zoom, handle_bear_background_shrink, despawn_shrinked).chain().run_if(in_state(LocalState::Zooming)))
            .add_systems(Update, cleanup.run_if(in_state(LocalState::Finished)))
            // .add_systems(Update, (t, update_mouse_pos).run_if(in_state(STATE)))
            // .add_observer(collision_handler)
            ;
    }
}

#[derive(Component, Clone, PartialEq, Eq, Hash)]
enum SpriteType {
    Black,
    BearQuestion,
    CJ,
    Joker,
    Miami,
    Cactus,
    FlappyBird,
    Bevy,
    Me,
}

#[derive(Resource)]
struct ToRemove{
    rm: Option<SpriteType>,
}

#[derive(Component)]
struct BackgroundMaterial;

fn spawn_texture_holder(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    return;
    commands.spawn((
        Name::new("Background"),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::Srgba(BLACK)))),
        Transform::from_xyz(0., 0., -4000.).with_scale(Vec3::splat(128.)),
        properties::HIGHRES_LAYERS,
        BackgroundMaterial,
    ));
}

fn resize_texture_holder(
    mut r: MessageReader<WindowResized>,
    mut query: Query<&mut Transform, With<BackgroundMaterial>>,
) {
    return;
    let Some(e) = r.read().last() else {return;};
    if e.width == 0.0 || e.height == 0.0 {return;}

    query.single_mut().unwrap().scale = vec3(e.width as f32, e.height as f32, 1.);
}

fn debug_cameras(
    cameras: Query<Entity, With<Camera>>,
    highres: Query<Entity, With<HighresCamera>>,
    ui_targets: Query<(Entity, &UiTargetCamera, Option<&Name>)>,
) {
    let cam_set: std::collections::HashSet<Entity> = cameras.iter().collect();
    tracing::info!(?cam_set, "camera entities");
    tracing::info!(highres = ?highres.iter().collect::<Vec<_>>(), "highres cameras");
    for (node_ent, target, name) in ui_targets.iter() {
        tracing::info!(
            node = ?node_ent,
            name = ?name.map(|n| n.as_str()),
            target = ?target.0,
            target_exists = cam_set.contains(&target.0),
            "UiTargetCamera mapping"
        );
    }
}

const INITIAL_TRANSLATION: [Vec2; 8] = [
    Vec2::new(0., 0.), // black & bear
    Vec2::new(-11.225, 22.825), // CJ
    Vec2::new(-1.776, -15.104), // clown
    Vec2::new(3.2, -19.2), // miami
    Vec2::new(5.04, -40.7), // cactus
    Vec2::new(-5.5, 12.3), // flappy
    Vec2::new(2.0, 24.5), // bavy
    Vec2::new(0., 0.), // Me
];

const INITIAL_SCALE: [f32; 8] = [
    1., // black & bear
    100., // CJ
    1000., // clown
    10000., // miami
    150000., // cactus
    10000000., // flappy
    200000000., // bavy
    10000000000., // Me
];

fn setup(
    mut cmd: Commands,
    cam: Query<Entity, With<HighresCamera>>,
    final_assets: Res<FinalAssets>,
    mut localstate: ResMut<NextState<LocalState>>,
) {
    localstate.set(LocalState::Bear);
    let cam = cam.iter().next().expect("No cam!");

    cmd.spawn((
        Name::new("Bear"),
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.question.clone(),
            ..default()
        },
        SpriteType::BearQuestion,
        Visibility::Hidden,
        ZIndex(2),
    ));
    cmd.spawn((
        Name::new("Black"),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.black.clone(),
            ..default()
        },
        SpriteType::Black,
        ZIndex(1),
    ));
    let mut cj_t = UiTransform::from_translation(
        Val2::percent(
            INITIAL_TRANSLATION[1].x * INITIAL_SCALE[1],
            INITIAL_TRANSLATION[1].y * INITIAL_SCALE[1]
        ));
    cj_t.scale = Vec2::splat(INITIAL_SCALE[1]);
    cmd.spawn((
        Name::new("CJ"),
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.cj.clone(),
            ..default()
        },
        cj_t,
        SpriteType::CJ,
        Transform::from_scale(Vec3::splat(10.)),
        ZIndex(0),
    ));
    let mut clown_t = UiTransform::from_translation( // change
        Val2::percent(
            INITIAL_TRANSLATION[2].x * INITIAL_SCALE[2], // change
            INITIAL_TRANSLATION[2].y * INITIAL_SCALE[2]  // change
        ));
    clown_t.scale = Vec2::splat(INITIAL_SCALE[2]); // change
    cmd.spawn((
        Name::new("Clown"), // change
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.clown.clone(), // change
            ..default()
        },
        clown_t, // change
        SpriteType::Joker, // change
        Transform::from_scale(Vec3::splat(10.)),
        ZIndex(-1), // change
    ));
    let mut miami_t = UiTransform::from_translation( // change
        Val2::percent(
            INITIAL_TRANSLATION[3].x * INITIAL_SCALE[3], // change x2
            INITIAL_TRANSLATION[3].y * INITIAL_SCALE[3]  // change x2
        ));
    miami_t.scale = Vec2::splat(INITIAL_SCALE[3]); // change x2
    cmd.spawn((
        Name::new("Miami"), // change
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.miami.clone(), // change
            ..default()
        },
        miami_t, // change
        SpriteType::Miami, // change
        Transform::from_scale(Vec3::splat(10.)),
        ZIndex(-2), // change
    ));
    let mut cactus_t = UiTransform::from_translation( // change
        Val2::percent(
            INITIAL_TRANSLATION[4].x * INITIAL_SCALE[4], // change x2
            INITIAL_TRANSLATION[4].y * INITIAL_SCALE[4]  // change x2
        ));
    cactus_t.scale = Vec2::splat(INITIAL_SCALE[4]); // change x2
    cmd.spawn((
        Name::new("Cactus"), // change
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.cactus.clone(), // change
            ..default()
        },
        cactus_t, // change
        SpriteType::Cactus, // change
        Transform::from_scale(Vec3::splat(10.)),
        ZIndex(-3), // change
    ));
    let mut bird_t = UiTransform::from_translation( // change
        Val2::percent(
            INITIAL_TRANSLATION[5].x * INITIAL_SCALE[5], // change x2
            INITIAL_TRANSLATION[5].y * INITIAL_SCALE[5]  // change x2
        ));
    bird_t.scale = Vec2::splat(INITIAL_SCALE[5]); // change x2
    cmd.spawn((
        Name::new("Bird"), // change
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.flappy_bird.clone(), // change
            ..default()
        },
        bird_t, // change
        SpriteType::FlappyBird, // change
        Transform::from_scale(Vec3::splat(10.)),
        ZIndex(-4), // change
    ));
    let mut bavy_t = UiTransform::from_translation( // change
        Val2::percent(
            INITIAL_TRANSLATION[6].x * INITIAL_SCALE[6], // change x2
            INITIAL_TRANSLATION[6].y * INITIAL_SCALE[6]  // change x2
        ));
    bavy_t.scale = Vec2::splat(INITIAL_SCALE[6]); // change x2
    cmd.spawn((
        Name::new("Bavy"), // change
        DespawnOnExit(STATE),
        UiTargetCamera(cam),
        ImageNode {
            image: final_assets.bavy.clone(), // change
            ..default()
        },
        bavy_t, // change
        SpriteType::Bevy, // change
        Transform::from_scale(Vec3::splat(10.)),
        ZIndex(-5), // change
    ));

    cmd.insert_resource(TimeCounter::default());
    cmd.insert_resource(Played::default());
    cmd.insert_resource(StartZoomout::default());
    cmd.insert_resource(ToRemove{rm: None});
}

const LIGHTS_OUT_TIMING: f32 = 0.1;
const QUESTION_TIMING: f32 = LIGHTS_OUT_TIMING + 1.;
const QUESTION_TO_ZOOMOUT_DELTA: f32 = 2.;
const START_ZOOMOUT_TIMING: f32 = QUESTION_TIMING + QUESTION_TO_ZOOMOUT_DELTA;

#[derive(Resource, Default)]
struct TimeCounter(f32);

#[derive(Resource, Default)]
struct Played([bool; 10]);

#[derive(Resource, Default)]
struct StartZoomout(bool);

fn timing_system(
    mut cmd: Commands,
    final_assets: Res<FinalAssets>,
    mut counter: ResMut<TimeCounter>,
    mut played: ResMut<Played>,
    time: Res<Time>,
    mut visibility: Query<(&mut Visibility, &SpriteType)>,
    mut start_zoomout: ResMut<StartZoomout>,
) {
    for (mut visibility, sprite_type) in visibility.iter_mut() {
        if counter.0 >= START_ZOOMOUT_TIMING && !played.0[2] {
            played.0[2] = true;
            start_zoomout.0 = true;
            cmd.spawn((
                DespawnOnExit(STATE),
                AudioPlayer(final_assets.zoom_out.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    ..default()
                },
            ));
        } else if counter.0 >= QUESTION_TIMING && !played.0[1] {
            if let SpriteType::BearQuestion = sprite_type {
                played.0[1] = true;
                cmd.spawn((
                    DespawnOnExit(STATE),
                    AudioPlayer(final_assets.freddy_appears.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Once,
                        ..default()
                    },
                ));
                *visibility = Visibility::Visible
            }
        } else if counter.0 >= LIGHTS_OUT_TIMING && !played.0[0] {
            played.0[0] = true;
            cmd.spawn((
                DespawnOnExit(STATE),
                AudioPlayer(final_assets.lights_out.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    ..default()
                },
            ));
        }
    }
    counter.0 += time.delta_secs();
}

fn start_zoom(
    visibility_q: Query<(&Visibility, &SpriteType)>,
    mut localstate: ResMut<NextState<LocalState>>,
    time: Res<Time>,
    mut timer: Local<f32>
) {
    if *timer >= QUESTION_TO_ZOOMOUT_DELTA {
        for (visibility, shrinking_type) in visibility_q {
            if shrinking_type == &SpriteType::BearQuestion && visibility == Visibility::Visible {
                localstate.set(LocalState::Zooming);
            }
        }
    } else {
        *timer += time.delta_secs();
    }
}

fn handle_bear_background_shrink(
    transform_q: Query<(&mut UiTransform, &SpriteType)>,
) {
    let mut target = Vec2::ZERO;
    for (transform, shrinking_type) in transform_q.iter() {
        if *shrinking_type == SpriteType::BearQuestion {
            target = transform.scale;
            break;
        }
    }
    for (mut transform, shrinking_type) in transform_q {
        if *shrinking_type == SpriteType::Black {
            transform.scale = target;
            break;
        }
    }
}

fn zoom(
    start_zoomout: Res<StartZoomout>,
    transform_q: Query<(&mut UiTransform, &SpriteType)>,
    time: Res<Time>,
    mut to_remove: ResMut<ToRemove>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut stop: Local<bool>,
) {
    let sprite_type2idx: HashMap<SpriteType, usize> = HashMap::from([
        (SpriteType::Black, 0),
        (SpriteType::BearQuestion, 0),
        (SpriteType::CJ, 1),
        (SpriteType::Joker, 2),
        (SpriteType::Miami, 3),
        (SpriteType::Cactus, 4),
        (SpriteType::FlappyBird, 5),
        (SpriteType::Bevy, 6),
        (SpriteType::Me, 7),
    ]);

    if keyboard_input.just_pressed(KeyCode::Space) { // todo: REMOVE
        if *stop {
            println!("STOP");
        } else {
            println!("CONTINUE");
        }
        *stop = !*stop;
    }
    
    const CHANGE: f32 = 0.6;
    const SCALE_THRESHOLD: f32 = 0.001;

    if start_zoomout.0 && !*stop {
        for (mut transform, shrinking_type) in transform_q {
            let scale_delta = transform.scale * CHANGE * time.dt();
            transform.scale -= scale_delta;
            let curr_id = *sprite_type2idx.get(shrinking_type).unwrap();
            transform.translation = Val2::percent(INITIAL_TRANSLATION[curr_id].x * transform.scale.x, INITIAL_TRANSLATION[curr_id].y * transform.scale.y);
            if transform.scale.x < SCALE_THRESHOLD {
                to_remove.rm = Some(shrinking_type.clone());
                println!("DELETING");
            }
        }
    }
}

fn despawn_shrinked(
    mut cmd: Commands,
    to_remove: Res<ToRemove>,
    transform_q: Query<(Entity, &SpriteType)>,
) {
    if let Some(type_to_rm) = to_remove.rm.clone() {
        for (entity, current_type) in transform_q {
            if type_to_rm == *current_type {
                cmd.entity(entity).despawn();
            }
        }
    }
}

fn cleanup(
    mut cmd: Commands,
) {
    cmd.remove_resource::<TimeCounter>();
    cmd.remove_resource::<Played>();
    cmd.remove_resource::<StartZoomout>();
    cmd.remove_resource::<ToRemove>();
}