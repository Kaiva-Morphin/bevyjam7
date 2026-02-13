use crate::{prelude::{AppState, LastState}, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;
use crate::global_music::plugin::NewBgMusic;

pub struct FlappyBirdPlugin;

// TODO!: TRANSITION EASINGS

const STATE: AppState = AppState::FlappyBird;
const NEXT_STATE: AppState = AppState::Geometry;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)]
#[states(scoped_entities)]
enum LocalState {
    #[default]
    InitialAnim,
    Game,
    Win,
}

impl Plugin for FlappyBirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LocalRes::default())
            .insert_resource(Pipes::default())
            .add_sub_state::<LocalState>()
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, tick_transition.run_if(in_state(LocalState::InitialAnim)))
            .add_systems(OnEnter(LocalState::Game), begin_game)
            .add_systems(Update, tick_game.run_if(in_state(LocalState::Game)))
            // .add_systems(Update, tick_defat.run_if(in_state(LocalState::Defeat)))
            .add_systems(Update, tick_win.run_if(in_state(LocalState::Win)))
            .add_systems(OnExit(STATE), cleanup)
            .add_observer(collision_handler)
            ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct FlappyBirdAssets {
    #[asset(path = "images/pipe.png")]
    pipe: Handle<Image>,
    #[asset(path = "images/flappy.png")]
    pacman: Handle<Image>,
    #[asset(path = "images/flappy_bird_transition.png")]
    transition_bg: Handle<Image>,
    #[asset(path = "images/flappy_bird_bg.png")]
    bg: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 792, tile_size_y = 1000, columns = 2, rows = 1))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "sounds/flappy/woosh.ogg")]
    woosh: Handle<AudioSource>,
    #[asset(path = "sounds/flappy/20 - Ccc- looptober-2021-variety-pack.ogg")]
    bg_music: Handle<AudioSource>,
}


#[derive(Component)]
pub struct Pacman;

#[derive(Component)]
pub struct TransitionScreen;

#[derive(Resource, Default)]
pub struct LocalRes {
    // for PARALLAX
    fake_x: f32,
}

fn setup(
    mut cmd: Commands,
    assets: Res<FlappyBirdAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut state: ResMut<LastState>,
) {
    state.state = STATE;

    cmd.spawn((
        NewBgMusic{handle: Some(assets.bg_music.clone()), instant_translation: false},
    ));

    cmd.spawn((
        DespawnOnExit(STATE),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Sprite {
            image: assets.bg.clone(),
            ..default()
        },
    ));
    cmd.insert_resource(LocalRes::default());
    
    cmd.insert_resource(Pipes{since_prev: 100., ..Default::default()});

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Bottom"),
        RigidBody::Kinematic,
        Collider::rectangle(FLAPPY_WIDTH, 10.0),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_translation(Vec3::new(0.0, -FLAPPY_HALF_HEIGHT, 0.0)),
    ));

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Top"),
        RigidBody::Kinematic,
        Collider::rectangle(FLAPPY_WIDTH, 10.0),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_translation(Vec3::new(0.0, FLAPPY_HALF_HEIGHT, 0.0)),
    ));

    meshes.add(Rectangle::new(50.0, 100.0));

    let r = meshes.add(Rectangle::new(FLAPPY_WIDTH, FLAPPY_HALF_HEIGHT*2. + 20.0));

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Transition"),
        Sprite {
            image: assets.transition_bg.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new(FLAPPY_WIDTH * 0.5, 0.0, 10.0)),
        TransitionScreen,
        children![(
            Mesh2d(r),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_translation(Vec3::new(-FLAPPY_WIDTH * 0.5 - 33.0, 0.0, 0.0))
        )]
    ));

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Pacman"),
        Transform::from_translation(Vec3::new(FLAPPY_WIDTH * 0.5, 0.0, 10.6)).with_scale(Vec3::splat(FLAPPY_SCALE)),
        Sprite {
            image: assets.pacman.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Pacman,
        Collider::circle(14.0 * (1.0 / FLAPPY_SCALE)),
        CollisionEventsEnabled,
        RigidBody::Dynamic,
        GravityScale(0.0),
    ));
}

fn begin_game (
    mut cmd: Commands,
    q: Query<Entity, With<Pacman>>
) {
    cmd.spawn((

    ));
    cmd.entity(q.iter().next().expect("No pacman!")).insert(GravityScale(FLAPPY_GRAVITY_AFFECT));
}

fn tick_transition(
    mut pacman: Query<&mut Transform, (With<Pacman>, Without<TransitionScreen>)>,
    mut transition: Query<&mut Transform, (With<TransitionScreen>, Without<Pacman>)>,
    mut res: ResMut<LocalRes>,
    t: Res<Time>,
    mut state: ResMut<NextState<LocalState>>
){
    let dt = t.delta_secs().min(MAX_DT);
    res.fake_x += FLAPPY_TRANSITION_SPEED * dt;
    for mut t in pacman.iter_mut() {
        t.translation.x -= (FLAPPY_TRANSITION_SPEED + FLAPPY_PARALLAX_SPEED) * dt;
        if t.translation.x <= FLAPPY_LEFT_BOUND {
            state.set(LocalState::Game);
        }
    }
    for mut t in transition.iter_mut() {
        t.translation.x -= FLAPPY_BG_TRANSITION_SPEED * dt;
    }
}


#[derive(Default, Resource)]
struct Pipes {
    pipes: Vec<Entity>,
    since_prev: f32,
    buffer: Vec<Entity>,
}

#[derive(Component)]
struct Pipe;

fn spawn_pipe(
    cmd: &mut Commands,
    assets: &FlappyBirdAssets,
) -> Entity {
   cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Pipe"),
        Pipe,
        Visibility::default(),
        InheritedVisibility::VISIBLE,
        GlobalTransform::default(),
        children![
            (
                Collider::rectangle(30.0 * (1.0 / FLAPPY_PIPE_SCALE), FLAPPY_HALF_HEIGHT * (1.0 / FLAPPY_PIPE_SCALE) * 1.7),
                Transform::from_translation(Vec3::new(0.0, -FLAPPY_HALF_HEIGHT * 0.5 - FLAPPY_PIPE_GAP, 5.0)).with_scale(Vec3::splat(FLAPPY_PIPE_SCALE)),
                CollisionEventsEnabled,
                Sensor,
                RigidBody::Kinematic,
                InheritedVisibility::VISIBLE,
                children!(
                    Sprite{
                        image: assets.pipe.clone(), ..default()
                    },
                )
            ),
            (
                Collider::rectangle(30.0 * (1.0 / FLAPPY_PIPE_SCALE), FLAPPY_HALF_HEIGHT * (1.0 / FLAPPY_PIPE_SCALE) * 1.7),
                Transform::from_translation(Vec3::new(0.0, FLAPPY_HALF_HEIGHT * 0.5 + FLAPPY_PIPE_GAP, 5.0)).with_scale(Vec3::splat(FLAPPY_PIPE_SCALE)),
                CollisionEventsEnabled,
                Sensor,
                RigidBody::Kinematic,
                InheritedVisibility::VISIBLE,
                children!(
                    Sprite{
                        image: assets.pipe.clone(), ..default()
                    },
                )
            ),
        ]
    )).id()
}


fn tick_game(
    mut state: ResMut<NextState<LocalState>>,
    mut pacman: Query<(&mut Transform, &mut LinearVelocity, &mut Sprite), (With<Pacman>, Without<Pipe>)>,
    mut pipe_q: Query<&mut Transform, (With<Pipe>, Without<Pacman>)>,
    mut res: ResMut<LocalRes>,
    t: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut at: Local<f32>,
    mut cmd: Commands,
    mut pipes: ResMut<Pipes>,
    assets: Res<FlappyBirdAssets>
){
    let mut rng = rand::rng();
    let dt = t.delta_secs().min(MAX_DT);
    let (mut t, mut v, mut s) = pacman.iter_mut().next().expect("No pacman!");
    res.fake_x += (FLAPPY_PARALLAX_SPEED - FLAPPY_BIRD_PROGRESS_SPEED) * dt;
    if keys.just_pressed(KeyCode::Space) {
        v.y = FLAPPY_BIRD_JUMP_STRENGTH;
        *at = 0.3;
        cmd.spawn((
            DespawnOnEnter(NEXT_STATE),
            AudioPlayer::new(assets.woosh.clone()),
        ));
    }
    t.translation += dt * FLAPPY_BIRD_PROGRESS_SPEED;
    let Some(a) = &mut s.texture_atlas else {return;};
    if *at > 0.0 {
        *at -= dt;
        a.index = 1;
    } else {
        a.index = 0;
    }
    if t.translation.x >= FLAPPY_RIGHT_BOUND {
        state.set(LocalState::Win);
    }

    pipes.since_prev += dt;
    for entity in pipes.pipes.clone().iter() {
        let Ok(mut e) = pipe_q.get_mut(*entity) else {continue;};
        e.translation.x -= FLAPPY_PIPE_SPEED * dt;
        if e.translation.x <= -FLAPPY_WIDTH * 0.5 {
            pipes.buffer.push(*entity);
        }
    }

    if pipes.since_prev > FLAPPY_PIPE_SPAWN_DELAY {
        pipes.since_prev = 0.0;
        let e = if let Some(p) = pipes.buffer.pop() {
            p
        } else {
            info!("Spawning new pipe");
            let e = spawn_pipe(&mut cmd, &assets);
            pipes.pipes.push(e);
            e
        };
        cmd.entity(e).insert(
            Transform::from_translation(vec3(FLAPPY_WIDTH, rng.random_range(-FLAPPY_PIPE_SPREAD..=FLAPPY_PIPE_SPREAD), 0.0))
        );
    }

}

// fn tick_defat(
//     mut cmd: Commands,
//     canvas: Res<camera::ViewportCanvas>,
// ){
//     // cmd.spawn(bevy::render::view::screenshot::Screenshot::image(canvas.image.clone()))
//     //     .observe(crate::games::plugin::await_screenshot_and_translate(AppState::Defeat));
// }

fn cleanup(
    mut cmd: Commands,
) {
    cmd.remove_resource::<Pipes>();
}

fn collision_handler(
    _e: On<CollisionStart>,
    // mut state: ResMut<NextState<LocalState>>,
    mut cmd: Commands,
    q: Query<Entity, With<Pacman>>,
    s: Res<State<AppState>>,
    ls: Option<Res<State<LocalState>>>,
    canvas: Res<camera::ViewportCanvas>,
    mut screenshot: ResMut<crate::properties::LastScreenshot>,
){
    if s.get() != &STATE {return;}
    let Some(l) = ls else {return;};
    if l.get() != &LocalState::Game {return;}
    let p = q.iter().next().expect("No pacman!");
    if _e.collider1 != p && _e.collider2 != p {return;}
    if screenshot.awaiting == false {
        cmd.spawn(bevy::render::view::screenshot::Screenshot::image(canvas.image.clone()))
            .observe(crate::properties::await_screenshot_and_translate(AppState::FakeEnd));
        screenshot.awaiting = true;
    }
    cmd.entity(q.iter().next().expect("No pacman!")).remove::<RigidBody>();
}

fn tick_win(
    mut state: ResMut<NextState<AppState>>,
    mut q: Query<(Entity, &mut Transform), With<Pacman>>,
    t: Res<Time>,
    mut cmd: Commands
) {
    let dt = t.delta_secs().min(MAX_DT);
    let (e, mut t) = q.iter_mut().next().expect("No pacman!");
    cmd.entity(e).remove::<RigidBody>();
    t.translation.x += dt * FLAPPY_BIRD_OUT_SPEED;
    if t.translation.x >= FLAPPY_WIDTH * 0.5 + 100.0 {
        state.set(NEXT_STATE)
    }
}