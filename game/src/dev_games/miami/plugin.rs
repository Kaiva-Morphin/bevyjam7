use std::time::Duration;

use avian2d::math::FRAC_PI_2;
use bevy::audio::{PlaybackMode, Volume};
use bevy_asset_loader::asset_collection::AssetCollection;
use camera::CameraController;
use games::global_music::plugin::NewBgMusic;

use super::{map::*, weapon::*};
use crate::dev_games::miami::bossfight::*;
use crate::{dev_games::miami::map::*, prelude::*};
use super::entity::*;
use super::shadows::*;
use super::player::*;
use super::dialog::*;

pub const STATE: AppState = AppState::Miami;
pub const NEXT_STATE: AppState = AppState::PacmanEnter;


#[derive(AssetCollection, Resource)]
pub struct MiamiAssets {
    #[asset(path = "maps/miami/map.tmx")]
    pub map: Handle<TiledMapAsset>,
    #[asset(path = "maps/miami/weapons.png")]
    pub weapons: Handle<Image>,

    #[asset(path = "maps/miami/pacman.png")]
    pub character: Handle<Image>,

    #[asset(path = "maps/miami/endoskeleton.png")]
    pub endoskeleton: Handle<Image>,
    #[asset(path = "maps/miami/copper_endoskeleton.png")]
    pub copper_endoskeleton: Handle<Image>,
    #[asset(path = "maps/miami/golden_endoskeleton.png")]
    pub golden_endoskeleton: Handle<Image>,


    #[asset(path = "maps/miami/bonnie.png")]
    pub bonnie: Handle<Image>,
    #[asset(path = "maps/miami/new_bonnie.png")]
    pub new_bonnie: Handle<Image>,
    #[asset(path = "maps/miami/chicka.png")]
    pub chica: Handle<Image>,
    #[asset(path = "maps/miami/new_chicka.png")]
    pub new_chica: Handle<Image>,

    #[asset(path = "maps/miami/faz.png")]
    pub freddy: Handle<Image>,
    
    #[asset(path = "maps/miami/decals.png")]
    pub decals: Handle<Image>,
    #[asset(path = "maps/miami/projectiles.png")]
    pub projectiles: Handle<Image>,

    #[asset(path = "maps/miami/door.png")]
    pub door: Handle<Image>,


    #[asset(path = "maps/miami/dialog_faz.png")]
    pub dialog_faz: Handle<Image>,
    #[asset(path = "maps/miami/dialog_pac.png")]
    pub dialog_pac: Handle<Image>,




    #[asset(path="sounds/miami/ACTION PACK 1 OGG_Magic Fx 7.ogg")]
    pub bg_music: Handle<AudioSource>,

    #[asset(path = "fonts/kaivs_minegram_v1.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "fonts/kaivs_minegram_v1-italic.ttf")]
    pub italic: Handle<Font>,

    #[asset(path = "sounds/miami/power_up.ogg")]
    pub powerup_sound: Handle<AudioSource>,

    #[asset(path = "sounds/novel/ururur.mp3")]
    pub ururur: Handle<AudioSource>,

}

pub struct MiamiPlugin;

impl Plugin for MiamiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TilemapShadow>()
            .register_type::<MiamiWeaponSpawner>()
            .register_type::<MiamiEntitySpawner>()
            .register_type::<BossEntrypointCollider>()
            .register_type::<EntrypointDialog>()
            .register_type::<BossDialog>()
            .register_type::<HorizontalDoor>()
            .register_type::<VerticalDoor>()
            .register_type::<BossfightSpawner>()
            .register_type::<FreddySpawner>()
            .register_type::<Weapon>()

            .add_sub_state::<FreddyFightStage>()

            .add_observer(setup_tilemap_shadows)
            .add_observer(on_weapon_spawnpoint)
            .add_observer(on_entity_spawnpoint)
            .add_observer(on_thrown_weapon_collision)
            .add_observer(on_pickup_weapon_collision)
            .add_observer(propagate_obstacles)
            .add_observer(on_projectile_hit)
            .add_observer(on_map_created)
            .add_observer(on_v_door)
            .add_observer(on_h_door)
            .add_observer(on_entrypoint_dialog_spawned)
            .add_observer(on_boss_entrypoint_spawned)
            .add_observer(on_boss_dialog_spawned)
            
            .add_systems(OnEnter(STATE), (setup))

            .add_systems(PostUpdate, setup_freddy_fight.run_if(in_state(FreddyFightStage::Idle)))
            .add_systems(OnEnter(FreddyFightStage::PreFreddy), start_freddy_enter_dialog)
            .add_systems(OnEnter(FreddyFightStage::PreFreddy), kill_endoskeletons)
            .add_systems(OnEnter(FreddyFightStage::Freddy), setup_freddy_fight2)
            .add_systems(PostUpdate, tick_bonnie_chicka_fight.run_if(in_state(FreddyFightStage::BonnieChicka)))
            .add_systems(PostUpdate, tick_freddy_fight.run_if(in_state(FreddyFightStage::Freddy)))

            .add_systems(Update, (
                bonnie_chicka_fight_attack,
                
                (cleanup_shadows, setup_shadows).chain(),
                update_projectile,
                player_look_at_cursor,
                update_controllers,
                (control_player, shoot, throw_weapon).chain(),
                tick_thrown,
                tick,

                update_chasers,
                chase,
                
                tick_dialog,
                update_screenshot,

                player_health_watcher,
                display_path,


                
                // update_shadows,
            ).run_if(in_state(STATE)))
            // .add_systems(PhysicsSchedule, update_shadows.after(tick_camera).in_set(PhysicsSystems::First))
            // .add_systems(PhysicsSchedule, update_shadows.after(tick_camera).in_set(NarrowPhaseSystems::Last))
            .add_systems(FixedLast, update_shadows.run_if(in_state(STATE)))
            .add_systems(PostUpdate, health_watcher.run_if(in_state(STATE)))
            // .add_systems(
            //     PhysicsSchedule,
            // update_shadows.in_set(ShadowSystems::Update).run_if(in_state(STATE))
            // )
            // .insert_resource(NavMeshesDebug(bevy::color::palettes::tailwind::RED_800.into()))

            // .add_systems(PostUpdate, (
            //     update_shadows,
            // ).after(TransformSystems::Propagate).run_if(in_state(STATE)))
            .add_systems(Update, (
                tick,
            ).run_if(in_state(STATE)))
            .add_systems(OnExit(STATE), cleanup)
            ;
    }
}


#[derive(Component)]
pub struct MiamiScreenshot(f32);
#[derive(Resource, Default)]
pub struct MiamiTransitionShooted;


fn setup(
    mut cmd: Commands,
    assets: Res<MiamiAssets>,
    // cam: Query<Entity, With<WorldCamera>>,
    last: Res<LastScreenshot>,
    mut latest: ResMut<LastState>,
    mut camera_controller: ResMut<CameraController>,
    completed: Option<Res<MiamiTransitionShooted>>,
){
    latest.state = STATE;

    cmd.spawn((
        NewBgMusic{handle: Some(assets.bg_music.clone()), instant_translation: false},
    ));
    cmd.spawn((
        DespawnOnExit(STATE),
        Name::new("Map"),
        TiledMap(assets.map.clone()),
    ));
    cmd.init_resource::<ShootedDialogs>();
    // let cam = cam.iter().next().expect("No cam!");
    // start_dialog(&mut cmd, &assets, cam, vec![
    //     ("YOU BASTARD!".to_string(), Speaker::Pacman),
    //     ("YOU BASTARD2!".to_string(), Speaker::Pacman),
    //     ("HELLO, PAC! I WILL KILL YOU!".to_string(), Speaker::Freddy),
    //     ("HELLO, PAC! I WILL KILL YOU!2".to_string(), Speaker::Freddy),
    // ]);
    if completed.is_some() {return;}
    cmd.init_resource::<MiamiTransitionShooted>();
    let Some(screenshot) = last.image.clone() else {return;};
    let tween = Tween::new(
        EaseFunction::QuinticOut,
        Duration::from_secs_f32(SCREENSHOT_TRANSITION_TIME),
        TransformRotationLens {
            start: Quat::from_rotation_x(0.0),
            end: Quat::from_rotation_x(FRAC_PI_2),
        }
    );
    cmd.spawn((
        Name::new("Screenshot"),
        DespawnOnExit(STATE),
        PlaybackSettings{
            mode: PlaybackMode::Once,
            volume: Volume::Linear(1.0),
            ..default()
        },
        AudioPlayer::new(assets.powerup_sound.clone()),
        TweenAnim::new(tween),
        MiamiScreenshot(0.0),
        Transform::from_translation(Vec3::new(0.0, 0.0, 500.0)),
        Sprite {
            image: screenshot,
            ..Default::default()
        },
        HIGHRES_LAYERS,
    ));
    camera_controller.follow_speed = 0.9;
    camera_controller.target_zoom = 0.9
}


const SCREENSHOT_TRANSITION_TIME: f32 = 0.5;


fn update_screenshot(
    mut screenshot: Query<(Entity, &mut MiamiScreenshot)>,
    mut cmd: Commands,
    dt: Res<Time>,
    p_q: Query<Entity, (With<Player>, With<PlayerDisabled>)>,
){
    if screenshot.iter().len() == 0 {return;}

    let dt = dt.dt();
    for (e, mut s) in screenshot.iter_mut() {
        s.0 += dt;
        if s.0 > SCREENSHOT_TRANSITION_TIME {
            cmd.entity(e).despawn();
            let Some(player) = p_q.iter().next() else {continue;};
            cmd.entity(player).remove::<PlayerDisabled>();
        }
    }
}


fn on_map_created(
    _event: On<TiledEvent<TilemapCreated>>,
    state: Res<State<AppState>>,
    mut map: Query<&mut Transform, With<TiledMap>>,
) {
    if state.get() != &STATE {return;};
    let Ok(mut map) = map.single_mut() else {return;};
    map.scale.z = 0.05;
    map.translation.z = 22.0;
}




fn tick(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<WorldCamera>>
){
    let Some(mut t) = camera.iter_mut().next() else {return;};
    // t.rotation.z = (time.elapsed_secs() * 0.7).sin() * 0.02; // ! TODO
}


fn cleanup(
    mut controller: ResMut<CameraController>,
    mut camera: Query<&mut Transform, With<WorldCamera>>,
    mut cmd: Commands,
    mut screenshot: ResMut<LastScreenshot>,
){
    cmd.remove_resource::<PlayerZeroHealthTicker>();
    cmd.remove_resource::<ShootedDialogs>();
    cmd.remove_resource::<BossfightDialog>();
    controller.follow_speed = 0.0;
    controller.target_zoom = 0.8;
    let Ok(mut t) = camera.single_mut() else {return;};
    t.rotation.z = 0.0;
    t.rotation.y = 0.0;
}


#[derive(Resource, Default)]
pub struct PlayerZeroHealthTicker(pub f32);
pub fn player_health_watcher(
    zh: Option<ResMut<PlayerZeroHealthTicker>>,
    time: Res<Time>,
    mut state: ResMut<NextState<AppState>>,
    mut screenshot: ResMut<LastScreenshot>,
    mut cmd: Commands,
) {
    let Some(mut zh) = zh else {
        return;
    };
    let dt = time.dt();
    zh.0 += dt;
    if zh.0 > DEFEAT_TIME {
        if screenshot.awaiting == false {
            cmd.spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(await_screenshot_and_translate(AppState::Defeat));
            screenshot.awaiting = true;
        }
    }
}

pub fn miami_player_layers() ->            CollisionLayers {CollisionLayers::from_bits(0b101000110, 0b101000111)}
pub fn miami_character_layers() ->         CollisionLayers {CollisionLayers::from_bits(0b010010010, 0b010010111)}
pub fn miami_dropped_weapon_layers() ->    CollisionLayers {CollisionLayers::from_bits(0b000011000, 0b000010011)}
pub fn miami_pickup_weapon_layers() ->     CollisionLayers {CollisionLayers::from_bits(0b001000000, 0b001000000)}
pub fn miami_weapon_layers() ->            CollisionLayers {CollisionLayers::from_bits(0b000000000, 0b000000000)}
pub fn miami_projectile_damager_layer() -> CollisionLayers {CollisionLayers::from_bits(0b010000001, 0b010000001)} 
pub fn miami_projectile_player_layer() ->  CollisionLayers {CollisionLayers::from_bits(0b100000000, 0b100000000)} 
pub fn miami_seeker_shapecast_layer() ->   CollisionLayers {CollisionLayers::from_bits(0b000000011, 0b000000011)} 
pub fn dialog_sensor_layer() ->            CollisionLayers {CollisionLayers::from_bits(0b000000100, 0b000000100)} 

pub fn red_blood() -> Color {Color::Srgba(Srgba::rgba_u8(200, 32, 61, 255))}
pub fn oil_blood() -> Color {Color::Srgba(Srgba::rgba_u8(30, 22, 64, 255))}
pub fn blood_rects() -> [Rect; 3] {
    [
        Rect::new(16.0, 32.0, 32.0, 48.0),
        Rect::new(0.0, 32.0, 16.0, 48.0),
        Rect::new(0.0, 0.0, 32.0, 32.0),
    ]
}
pub fn front_body_rect() -> Rect {Rect::new(48.0, 0.0, 80.0, 64.0)}
pub fn back_body_rect() -> Rect {Rect::new(80.0, 0.0, 112.0, 64.0)}

pub const BLOOD_Z_TRANSLATION : f32 = -6.0;
pub const BODY_Z_TRANSLATION : f32 = -4.0;
pub const THROWN_DAMAGE_MULTIPLIER: f32 = 0.0071428571;

pub const DEFEAT_TIME: f32 = 1.0;

pub const CHASER_RANDOM_RADIUS: f32 = 100.0;

pub const SHOTGUN_BULLET_RADIUS: f32 = 0.4;
pub const SHOTGUN_BULLET_COUNT: usize = 6;

