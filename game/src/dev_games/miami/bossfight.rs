use camera::CameraController;
use rand::Rng;

use crate::{dev_games::miami::{entity::{CharacterController, InvincibleCharacter, MiamiEntity, Player, spawn_entity}, map::BossEntrypointCollider, plugin::{MiamiAssets, STATE}}, prelude::*};


#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct FreddySpawner;





#[derive(Component, Default)]
pub struct DisabledFreddyAi;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)] pub enum FreddyFightStage {
    #[default]
    Idle,
    BonnieChicka,
    PreFreddy,
    Freddy,
    Finished
}

#[derive(Component, Default)]
pub struct BossFightWait;

#[derive(Component, Default)]
pub struct BossFightStandAi;

#[derive(Component, Default)]
pub struct BossFightChaseAi;


#[derive(Component)]
pub struct FighterBonnie;

#[derive(Component)]
pub struct FighterChicka;

#[derive(Component)]
pub struct FighterFreddy;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct BossfightSpawner;

// #[derive(Component, Default)]
// pub struct 

pub fn setup_freddy_fight(
    c_q: Query<(Entity, &CharacterController), (Without<FighterChicka>, Without<FighterBonnie>, Without<FighterFreddy>, Without<BossFightWait>)>,
    mut cmd: Commands,
    mut state: ResMut<NextState<FreddyFightStage>>,
    f: Query<(), (With<FighterFreddy>, With<BossFightWait>)>,
    c: Query<(), (With<FighterChicka>, Without<BossFightWait>)>,
    b: Query<(), (With<FighterBonnie>, Without<BossFightWait>)>,
){
    for (e, c) in c_q {
        match c.character {
            MiamiEntity::NewChicka => 
            {
                cmd.entity(e).insert((Name::new("Chicka"), FighterChicka, BossFightStandAi)).remove::<InvincibleCharacter>();
            }
            MiamiEntity::NewBonnie => {
                cmd.entity(e).insert((Name::new("Bonnie"), FighterBonnie, BossFightStandAi)).remove::<InvincibleCharacter>();
            }
            MiamiEntity::Freddy => {
                cmd.entity(e).insert((Name::new("Freddy"), FighterFreddy, BossFightStandAi, BossFightWait));
            }
            _ => {}
        }
    }
    if !(f.is_empty() || c.is_empty() || b.is_empty()) {
        state.set(FreddyFightStage::BonnieChicka);
    }
}

pub fn begin_bossfight(
    cmd: &mut  Commands,
    e: &Query<Entity, (With<BossFightWait>, Without<FighterFreddy>)>,
    q: &Query<Entity, With<BossEntrypointCollider>>,
) {
    for e in e.iter() {
        cmd.entity(e).remove::<BossFightWait>();
    }
    super::map::block_bossroom(cmd, q);
}

pub fn tick_bonnie_chicka_fight(
    chicka: Query<(Entity, &CharacterController), (With<FighterChicka>, Without<BossFightStandAi>, Without<BossFightWait>)>,
    bonnie: Query<(Entity, &CharacterController), (With<FighterBonnie>, Without<BossFightStandAi>, Without<BossFightWait>)>,
    standing_chicka: Query<(Entity, &CharacterController), (With<FighterChicka>, With<BossFightStandAi>, Without<BossFightWait>)>,
    standing_bonnie: Query<(Entity, &CharacterController), (With<FighterBonnie>, With<BossFightStandAi>, Without<BossFightWait>)>,
    mut state: ResMut<NextState<FreddyFightStage>>,
    mut cmd: Commands,
    time: Res<Time>,
    mut since_prev: Local<f32>,
    q: Query<&GlobalTransform, With<BossfightSpawner>>,
    assets: Res<MiamiAssets>,
    mut camera_controller: ResMut<CameraController>,
){
    let mut rng = rand::rng();
    let dt = time.dt();
    let chicka = chicka.single().ok();
    let bonnie = bonnie.single().ok();
    let standing_chicka = standing_chicka.single().ok();
    let standing_bonnie = standing_bonnie.single().ok();
    if chicka.is_none() && bonnie.is_none() && standing_chicka.is_none() && standing_bonnie.is_none() {
        state.set(FreddyFightStage::PreFreddy);
        return;
    }
    if standing_chicka.is_none() || standing_bonnie.is_none() {
        *since_prev -= dt;
        if *since_prev < 0.0 {
            *since_prev = BONNIE_CHICKA_ENDOSKELETON_SPAWN_DELAY;
            let idx = rng.random_range(0..4);
            let gt= q.iter().collect::<Vec<_>>()[idx];
            let roll = rng.random_range(0..TOTAL_WEIGHT);
            let entity_type = match roll {
                0..WEIGHT_COPPER =>
                    MiamiEntity::CopperEndoskeleton,
                WEIGHT_COPPER..COPPER_GOLD =>
                    MiamiEntity::GoldenEndoskeleton,
                _ =>
                    MiamiEntity::Endoskeleton,
            };
            spawn_entity(&mut cmd, entity_type, &assets, &mut camera_controller, Transform::from_translation(gt.translation()), Vec2::ZERO);
        }
    }
    if let Some((chicka, c)) = standing_chicka {
        if c.hp <= CHICKA_CHASE_THRESHOLD {
            cmd.entity(chicka).remove::<BossFightStandAi>();
            return;
        };
    }
    if let Some((bonnie, c)) = standing_bonnie {
        if c.hp <= BONNIE_CHASE_THRESHOLD {
            cmd.entity(bonnie).remove::<BossFightStandAi>();
            return;
        };
    }
}

pub fn kill_endoskeletons(
    chars: Query<&mut CharacterController>,
){
    for mut c in chars {
        match c.character {
            MiamiEntity::Endoskeleton
            | MiamiEntity::CopperEndoskeleton
            | MiamiEntity::GoldenEndoskeleton => {
                c.hp = 0.0;
            }
            _ => {}
        }
    }
}

pub fn bonnie_chicka_fight_attack(
    mut fighters: Query<(&mut CharacterController, &GlobalTransform, Option<&BossFightStandAi>), (Without<Player>, Without<BossFightWait>)>,
    player: Query<&GlobalTransform, With<Player>>,
) {
    let Some(player) = player.iter().next() else {return;};
    for (mut fighter, gt, ai) in fighters.iter_mut() {
        fighter.shoot = true;
        if ai.is_some() {
            fighter.look_dir = (player.translation() - gt.translation()).normalize_or_zero().truncate();   
        }
    }
}

pub fn setup_freddy_fight2(
    mut cmd: Commands,
    q: Query<Entity, With<FighterFreddy>>,
){
    for e in q {
        cmd.entity(e).remove::<(InvincibleCharacter, BossFightWait)>();
    }
}

pub fn tick_freddy_fight(
    mut cmd: Commands,
    time: Res<Time>,
    standing_freddy: Query<
        (Entity, &CharacterController),
        (With<FighterFreddy>, With<BossFightStandAi>, Without<BossFightWait>)
    >,
    freddy: Query<
        (Entity, &CharacterController),
        (With<FighterFreddy>, Without<BossFightStandAi>, Without<BossFightWait>)
    >,
    q: Query<&GlobalTransform, With<BossfightSpawner>>,
    assets: Res<MiamiAssets>,
    mut camera_controller: ResMut<CameraController>,
    mut since_prev: Local<f32>,
){
    let mut rng = rand::rng();
    let dt = time.dt();

    let standing_freddy = standing_freddy.single().ok();
    let freddy = freddy.single().ok();

    if let Some((entity, c)) = standing_freddy {
        if c.hp <= FREDDY_CHASE_THRESHOLD {
            cmd.entity(entity).remove::<BossFightStandAi>();
            return;
        }
    }

    let spawn_delay = if standing_freddy.is_some() {
        FREDDY1_ENDOSKELETON_SPAWN_DELAY
    } else if freddy.is_some() {
        FREDDY2_ENDOSKELETON_SPAWN_DELAY
    } else {
        return;
    };

    *since_prev -= dt;
    if *since_prev >= 0.0 {
        return;
    }
    *since_prev = spawn_delay;

    let spawners: Vec<_> = q.iter().collect();
    let gt = spawners[rng.random_range(0..spawners.len())];

    let roll = rng.random_range(0..TOTAL_WEIGHT);
    let entity_type = match roll {
        0..WEIGHT_COPPER =>
            MiamiEntity::CopperEndoskeleton,
        WEIGHT_COPPER..COPPER_GOLD =>
            MiamiEntity::GoldenEndoskeleton,
        _ =>
            MiamiEntity::Endoskeleton,
    };

    spawn_entity(
        &mut cmd,
        entity_type,
        &assets,
        &mut camera_controller,
        Transform::from_translation(gt.translation()),
        Vec2::ZERO,
    );
}

const CHICKA_CHASE_THRESHOLD: f32 = 100.0;
const BONNIE_CHASE_THRESHOLD: f32 = 100.0;
const FREDDY_CHASE_THRESHOLD: f32 = 100.0;
const BONNIE_CHICKA_ENDOSKELETON_SPAWN_DELAY: f32 = 10.0;
const FREDDY1_ENDOSKELETON_SPAWN_DELAY: f32 = 5.0;
const FREDDY2_ENDOSKELETON_SPAWN_DELAY: f32 = 3.0;

const WEIGHT_COPPER: u32 = 50;
const WEIGHT_GOLDEN: u32 = 10;
const WEIGHT_NORMAL: u32 = 40;
const COPPER_GOLD: u32 = WEIGHT_COPPER + WEIGHT_GOLDEN;
const TOTAL_WEIGHT: u32 = WEIGHT_COPPER + WEIGHT_GOLDEN + WEIGHT_NORMAL;
