use crate::{games::plugin::AppState, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;

pub struct FlappyBirdPlugin;

// TODO!: TRANSITION EASINGS

const STATE: AppState = AppState::FlappyBird;
const NEXT_STATE: AppState = AppState::PacmanEnter;


const WIDTH : f32 = 576.0;
const HALF_HEIGHT : f32 = 250.0 / 2.0;
const SCALE : f32 = 1.0;

const LEFT_BOUND : f32 = -WIDTH / 2.0 + 120.0;
const RIGHT_BOUND : f32 = WIDTH / 2.0 - 200.0;

const PACMAN_TRANSITION_SPEED : f32 = 500.;
const BG_TRANSITION_SPEED : f32 = 800.;
const PARALLAX_SPEED : f32 = 50.0;
const PACMAN_PROGRESS_SPEED : f32 = 25.0;
const PACMAN_OUT_SPEED : f32 = 500.0;

const PIPE_SPEED : f32 = 250.0;
const PIPE_SPAWN_DELAY : f32 = 1.5;
const PIPE_GAP : f32 = 25.0;
const PIPE_SPREAD : f32 = 40.0;

const GRAVITY_AFFECT : f32 = 50.0;
const PACMAN_JUMP_STRENGTH : f32 = 200.0;

const DEATH_DELAY : f32 = 1.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)]
#[states(scoped_entities)]
enum LocalState {
    #[default]
    InitialAnim,
    Game,
    Defeat,
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
            .add_systems(Update, tick_defat.run_if(in_state(LocalState::Defeat)))
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
    #[asset(path = "images/pacman.png")]
    pacman: Handle<Image>,
    #[asset(path = "images/flappy_bird_transition.png")]
    transition_bg: Handle<Image>,
    #[asset(path = "images/flappy_bird_bg.png")]
    bg: Handle<Image>,
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    cmd.spawn((
        DespawnOnExit(STATE),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Sprite {
            image: assets.bg.clone(),
            ..default()
        },
    ));
    cmd.insert_resource(LocalRes::default());
    cmd.insert_resource(Pipes::default());

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Bottom"),
        RigidBody::Kinematic,
        Collider::rectangle(WIDTH, 10.0),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_translation(Vec3::new(0.0, -HALF_HEIGHT, 0.0)),
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Top"),
        RigidBody::Kinematic,
        Collider::rectangle(WIDTH, 10.0),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_translation(Vec3::new(0.0, HALF_HEIGHT, 0.0)),
    ));

    meshes.add(Rectangle::new(50.0, 100.0));

    let r = meshes.add(Rectangle::new(WIDTH, HALF_HEIGHT*2. + 20.0));

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Transition"),
        Sprite {
            image: assets.transition_bg.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new(WIDTH * 0.5, 0.0, 10.0)),
        TransitionScreen,
        children![(
            Mesh2d(r),
            MeshMaterial2d(materials.add(Color::BLACK)),
            Transform::from_translation(Vec3::new(-WIDTH * 0.5 - 33.0, 0.0, 0.0))
        )]
    ));

    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Pacman"),
        Transform::from_translation(Vec3::new(WIDTH * 0.5, 0.0, 10.6)).with_scale(Vec3::splat(SCALE)),
        Sprite {
            image: assets.pacman.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 3,
            }),
            ..default()
        },
        Pacman,
        Collider::circle(8.0),
        CollisionEventsEnabled,
        RigidBody::Dynamic,
        GravityScale(0.0),
    ));
}

fn begin_game (
    mut cmd: Commands,
    q: Query<Entity, With<Pacman>>
) {
    cmd.entity(q.iter().next().expect("No pacman!")).insert(GravityScale(GRAVITY_AFFECT));
}

fn tick_transition(
    mut pacman: Query<&mut Transform, (With<Pacman>, Without<TransitionScreen>)>,
    mut transition: Query<&mut Transform, (With<TransitionScreen>, Without<Pacman>)>,
    mut res: ResMut<LocalRes>,
    t: Res<Time>,
    mut state: ResMut<NextState<LocalState>>
){
    let dt = t.delta_secs().min(MAX_DT);
    res.fake_x += PACMAN_TRANSITION_SPEED * dt;
    for mut t in pacman.iter_mut() {
        t.translation.x -= (PACMAN_TRANSITION_SPEED + PARALLAX_SPEED) * dt;
        if t.translation.x <= LEFT_BOUND {
            state.set(LocalState::Game);
        }
    }
    for mut t in transition.iter_mut() {
        t.translation.x -= BG_TRANSITION_SPEED * dt;
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
                Collider::rectangle(10.0, HALF_HEIGHT),
                Transform::from_translation(Vec3::new(0.0, -HALF_HEIGHT * 0.5 - PIPE_GAP, 5.0)),
                CollisionEventsEnabled,
                Sensor,
                RigidBody::Kinematic,
                InheritedVisibility::VISIBLE,
                children!(
                    Sprite{
                        image: assets.pipe.clone(), ..default()
                    }
                )
            ),
            (
                Collider::rectangle(10.0, HALF_HEIGHT),
                Transform::from_translation(Vec3::new(0.0, HALF_HEIGHT * 0.5 + PIPE_GAP, 5.0)),
                CollisionEventsEnabled,
                Sensor,
                RigidBody::Kinematic,
                InheritedVisibility::VISIBLE,
                children!(
                    Sprite{
                        image: assets.pipe.clone(), ..default()
                    }
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
    res.fake_x += (PARALLAX_SPEED - PACMAN_PROGRESS_SPEED) * dt;
    if keys.just_pressed(KeyCode::Space) {
        v.y = PACMAN_JUMP_STRENGTH;
        *at = 0.3;
    }
    t.translation += dt * PACMAN_PROGRESS_SPEED;
    let Some(a) = &mut s.texture_atlas else {return;};
    if *at > 0.0 {
        *at -= dt;
        a.index = 4;
    } else {
        a.index = 3;
    }
    if t.translation.x >= RIGHT_BOUND {
        state.set(LocalState::Win);
    }

    pipes.since_prev += dt;
    for entity in pipes.pipes.clone().iter() {
        let Ok(mut e) = pipe_q.get_mut(*entity) else {continue;};
        e.translation.x -= PIPE_SPEED * dt;
        if e.translation.x <= -WIDTH * 0.5 {
            pipes.buffer.push(*entity);
        }
    }

    if pipes.since_prev > PIPE_SPAWN_DELAY {
        pipes.since_prev = 0.0;
        let e = if let Some(p) = pipes.buffer.pop() {
            p
        } else {
            info!("Spawning new pipe");
            let e = spawn_pipe(&mut cmd, &assets);
            pipes.pipes.push(e);
            e
        };
        let r = rng.random_range(-PIPE_SPREAD..=PIPE_SPREAD);
        info!("{r}");
        cmd.entity(e).insert(
            // Transform::from_translation(vec3(WIDTH, rng.random_range(-PIPE_SPREAD..=PIPE_SPREAD), 0.0))
            Transform::from_translation(vec3(WIDTH, r, 0.0))
        );
    }

}

fn tick_defat(
    mut t: Local<f32>,
    time: Res<Time>,
    mut state: ResMut<NextState<AppState>>,
){
    let dt = time.delta_secs().min(MAX_DT);
    *t += dt;
    if *t >= DEATH_DELAY {
        state.set(AppState::PacmanEnter);
    }
}

fn cleanup() {}

fn collision_handler(
    _e: On<CollisionStart>,
    mut state: ResMut<NextState<LocalState>>,
    mut cmd: Commands,
    q: Query<Entity, With<Pacman>>
){
    let p = q.iter().next().expect("No pacman!");
    if _e.collider1 != p && _e.collider2 != p {return;}
    state.set(LocalState::Defeat);
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
    t.translation.x += dt * PACMAN_OUT_SPEED;
    if t.translation.x >= WIDTH * 0.5 + 100.0 {
        state.set(NEXT_STATE)
    }
}