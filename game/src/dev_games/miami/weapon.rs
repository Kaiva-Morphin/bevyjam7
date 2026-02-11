use avian2d::math::Vector;
use bevy::prelude::*;
use games::prelude::AppState;
use rand::Rng;
use crate::dev_games::miami::entity::{CharacterComponents, CharacterController, CharacterPivotPoint, CharacterSprite, Player};
use crate::dev_games::miami::plugin::{miami_dropped_weapon_layers, miami_pickup_weapon_layers};
use crate::dev_games::miami::shadows::ShadowCaster;
use crate::prelude::*;
use crate::dev_games::miami::{plugin::{MiamiAssets, STATE}, shadows::ShadowInit};



#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct MiamiWeaponSpawner {
    weapon_type: WeaponType
}


#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub enum WeaponType {
    #[default]
    Pistol,
    Axe,
    Baguette,

    EnemyFists,
}

#[derive(Component, Default)]
pub struct Weapon {
    pub rect: Rect,
    pub held_rect: Rect,
    pub held_offset: Vec3,

    pub attack_rect: Rect,
    pub attack_offset: Vec3,
    
    pub char_rect: Rect,
    pub char_offset: Vec3,

    pub attack_char_rect: Rect,
    pub attack_char_offset: Vec3,

    pub ammo: u32,
    pub cooldown: f32,
    pub anim_time: f32,
    pub t: f32,

    pub weapon_type: WeaponType
}
#[derive(Component)]
pub struct WeaponComponents {
    pub sprite: Entity,
}


#[derive(Component)]
pub struct ArmedCharacter(Entity);

#[derive(Component)]
pub struct ThrownWeapon;

#[derive(Component)]
pub struct ReadyToPickUpWeapon;

#[derive(Component)]
pub struct WeaponSprite;

#[derive(Component)]
pub struct WeaponProjectile;


impl WeaponType {
    pub fn to_weapon(&self) -> Weapon {
        match self {
            WeaponType::Pistol => Weapon {
                weapon_type: WeaponType::Pistol,

                rect: Rect::new(0.0, 16.0, 16.0, 32.0),
                held_rect: Rect::new(0.0, 16.0, 16.0, 32.0),
                held_offset: vec3(-2., -25., -0.05),
                
                char_rect: Rect::new(32.0, 0.0, 48.0, 48.0),
                char_offset: vec3(0., -3., 0.),

                ammo: 30,
                cooldown: 0.2,
                anim_time: 0.2,

                attack_char_rect: Rect::new(32.0, 0.0, 48.0, 48.0),
                attack_char_offset: vec3(0., -3., 0.),

                attack_offset: vec3(-2., -25., -0.05),
                attack_rect: Rect::new(16.0, 16.0, 32.0, 32.0),

                ..Default::default()
            },
            WeaponType::Axe => Weapon {
                weapon_type: WeaponType::Axe,

                rect: Rect::new(32.0, 0.0, 64.0, 16.0),
                held_rect: Rect::new(0.0, 0.0, 32.0, 16.0),
                held_offset: vec3(-5., -6., -0.3),
                //
                
                char_rect: Rect::new(0.0, 32.0, 32.0, 48.0),
                char_offset: Vec3::ZERO,

                ammo: u32::MAX,
                cooldown: 0.2,
                anim_time: 0.2,

                attack_rect: Rect::new(32.0, 16.0, 64.0, 32.0),
                attack_offset: vec3(3., -10.0,  -0.3),
                
                attack_char_rect: Rect::new(0.0, 48.0, 32.0, 64.0),
                attack_char_offset: Vec3::ZERO,
                ..Default::default()
            },
            WeaponType::Baguette => Weapon {
                weapon_type: WeaponType::Baguette,

                rect: Rect::new(0.0, 48.0, 32.0, 64.0),
                held_rect: Rect::new(0.0, 32.0, 32.0, 48.0),
                held_offset: vec3(-5., -6., -0.3),
                //
                
                char_rect: Rect::new(0.0, 32.0, 32.0, 48.0),
                char_offset: Vec3::ZERO,

                ammo: u32::MAX,
                cooldown: 0.1,
                anim_time: 0.1,

                attack_rect: Rect::new(32.0, 32.0, 64.0, 48.0),
                attack_offset: vec3(3., -10.0,  -0.3),
                
                attack_char_rect: Rect::new(0.0, 48.0, 32.0, 64.0),
                attack_char_offset: Vec3::ZERO,
                ..Default::default()
            },

            WeaponType::EnemyFists => Weapon {
                weapon_type: WeaponType::EnemyFists,

                rect: Rect::new(0.0, 0.0, 16.0, 16.0),
                held_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
                held_offset: Vec3::ZERO,
                
                char_rect: Rect::new(0.0, 32.0, 32.0, 64.0),
                char_offset: vec3(0., -8., 0.),

                ammo: u32::MAX,
                cooldown: 0.1,
                anim_time: 0.1,

                attack_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
                attack_offset: Vec3::ZERO,
                
                attack_char_rect: Rect::new(0.0, 64.0, 32.0, 96.0),
                attack_char_offset: vec3(0., -8., 0.),
                ..Default::default()
            }
        }
    }
}

impl Weapon {
    pub fn on_shoot(
        &self,
        cmd: &mut Commands,
        dir: Vec2,
        is_enemy: bool
    ) -> Entity {
        let damage_layer = if is_enemy {} else {};
        match self.weapon_type {
            WeaponType::Pistol => {
                cmd.spawn((

                )).id()
            },
            _ => unimplemented!()
        }
    }
}

pub fn on_weapon_spawnpoint(
    point: On<Add, MiamiWeaponSpawner>,
    q: Query<(&MiamiWeaponSpawner, &Transform)>,
    mut cmd: Commands,
    assets: Res<MiamiAssets>,
    state: Res<State<AppState>>,
){
    if state.get() != &STATE {return;}
    let Ok((spawner, transform)) = q.get(point.entity) else {return;};
    let wpn: Weapon = spawner.weapon_type.to_weapon();
    let sprite = cmd.spawn((
        Visibility::default(),
        WeaponSprite,
        Transform::default(),
        Sprite {
            image: assets.weapons.clone(),
            rect: Some(wpn.rect.clone()),
            ..Default::default()
        },
    )).id();
    let w = cmd.spawn((
        Sensor,
        CollisionEventsEnabled,
        Collider::circle(8.0),
        Name::new("Weapon"),
        DespawnOnExit(STATE),
        ReadyToPickUpWeapon,
        Visibility::default(),
        GlobalTransform::default(),
        (
            LockedAxes::ROTATION_LOCKED,
            Friction::ZERO,
            miami_pickup_weapon_layers(),
            LinearDamping(3.0),
            GravityScale(0.0),
            Transform::from_translation(transform.translation),
        ),
        WeaponComponents{sprite},
        wpn,
    )).id();
    cmd.entity(w).add_child(sprite);
}


pub fn on_pickup_weapon_collision(
    event: On<CollisionStart>,
    state: Res<State<AppState>>,
    pickupable: Query<(&Weapon, &Children), With<ReadyToPickUpWeapon>>,
    characters: Query<&Children, (With<CharacterController>, With<Player>, Without<ArmedCharacter>)>, // todo!: Anyone?
    pivots: Query<&Children, With<CharacterPivotPoint>>,
    mut sprite: Query<(&mut Transform, &mut Sprite), With<CharacterSprite>>,
    mut weapon_sprite: Query<&mut Sprite, Without<CharacterSprite>>,
    mut cmd: Commands
){
    if state.get() != &STATE {return;}
    let weapon_entity = event.collider1;
    let Ok((weapon, weapon_children)) = pickupable.get(event.collider1) else {return;};
    let Ok(children) = characters.get(event.collider2) else {return;};
    for maybe_pivot in children.iter() {
        let Ok(c) = pivots.get(maybe_pivot) else {continue;};
        for c in c.iter() {
            let Ok((mut t, mut sprite)) = sprite.get_mut(c) else {continue;};
            sprite.rect = Some(weapon.char_rect.clone());
            t.translation = weapon.char_offset.clone();
        }
        for c in weapon_children.iter() {
            let Ok(mut sprite) = weapon_sprite.get_mut(c) else {continue;};
            sprite.rect = Some(weapon.held_rect.clone());
            cmd.entity(c).insert(ShadowInit);
        }
        let w = cmd.entity(weapon_entity)
            .remove::<(ReadyToPickUpWeapon, Sensor)>()
            .insert((Transform::from_translation(weapon.held_offset), WeaponOf(event.collider2))).id();
        cmd.entity(maybe_pivot).add_child(w);
        cmd.entity(event.collider2).insert(ArmedCharacter(w));
    };
}


#[derive(Component)]
pub struct WeaponOf(Entity);


pub fn throw_weapon(
    mut cmd: Commands,
    characters: Query<(&Children, &CharacterController), With<ArmedCharacter>>,
    pivots: Query<&Children, With<CharacterPivotPoint>>,
    mut sprites: Query<(&mut Transform, &mut Sprite, &CharacterSprite), Without<WeaponSprite>>,
    weapons: Query<(Entity, &WeaponOf, &Weapon, &Children)>,
    mut weapon_sprites: Query<&mut Sprite, (With<WeaponSprite>, Without<CharacterSprite>)>,
) {
    let mut r = rand::rng();
    for (wpn, armed, w, c) in weapons.iter() {
        let owner = armed.0;
        let Ok((child, controller)) = characters.get(owner) else {continue;};
        if !controller.throw {continue;}
        for c in c {
            let Ok(mut sprite) = weapon_sprites.get_mut(*c) else {continue;};
            sprite.rect =  Some(w.rect.clone());
        }
        for maybe_pivot in child {
            let Ok(pivot) = pivots.get(*maybe_pivot) else {continue;};
            for maybe_sprite in pivot.iter() {
                let Ok((mut t, mut sprite, character)) = sprites.get_mut(maybe_sprite) else {continue;};
                t.translation = character.default_offset.clone();
                sprite.rect = Some(character.default_rect.clone());
                // let vel = ;
                cmd.entity(wpn).remove::<
                    WeaponOf
                >().insert((
                    RigidBody::Dynamic,
                    ThrownWeapon,
                    Restitution::new(0.5),
                    LockedAxes::new(),
                    AngularVelocity{0: r.random_range(0.5..=1.2)},
                    miami_dropped_weapon_layers(),
                    LinearVelocity(Vector::new(controller.look_dir.x, controller.look_dir.y) * 400.0),
                ));
                cmd.entity(wpn).remove_parent_in_place();
                cmd.entity(owner).remove::<ArmedCharacter>();
            }
        }
    }
}

pub fn tick_thrown(
    thrown: Query<(Entity, &LinearVelocity, &Children, &Weapon), With<ThrownWeapon>>,
    mut sprites: Query<&WeaponSprite>,
    mut cmd: Commands,
) {
    for (e, vel, c, _w) in thrown.iter() {
        if vel.0.length_squared() < 10.0 {
            cmd.entity(e).remove::<(
                ThrownWeapon,
                RigidBody
            )>().insert((
                ReadyToPickUpWeapon,
                LinearVelocity::ZERO,
                Sensor,
                LockedAxes::ROTATION_LOCKED,
                miami_pickup_weapon_layers()
            ));
            for c in c {
                let Ok(_) = sprites.get_mut(*c) else {continue;};
                cmd.entity(*c).remove::<ShadowCaster>();       
            }
        }
    }
}

pub fn shoot(
    characters: Query<(&Children, &CharacterController, &CharacterComponents, &ArmedCharacter)>,
    mut weapons: Query<(&mut Weapon, &mut Transform, &WeaponComponents), Without<CharacterSprite>>,
    mut weapon_sprite: Query<(&mut Sprite), (With<WeaponSprite>, Without<CharacterSprite>)>,
    mut char_sprite: Query<(&mut Sprite, &mut Transform), (With<CharacterSprite>, Without<WeaponSprite>, Without<WeaponComponents>)>,
    mut cmd: Commands,
    time: Res<Time>,
){
    let dt = time.dt();
    for (child, controller, cc, a) in characters.iter() {
        let Ok((mut w,mut w_transform, wc)) = weapons.get_mut(a.0) else {continue;};
        // info!("Weapon!");
        let Ok((mut w_sprite)) = weapon_sprite.get_mut(wc.sprite) else {continue;};
        // info!("Weapon sprite!");
        let Ok((mut c_sprite, mut c_transform)) = char_sprite.get_mut(cc.sprite) else {continue;};
        // info!("Character sprite!");

        if w.t > w.cooldown - w.anim_time {
            w_sprite.rect = Some(w.attack_rect.clone());
            w_transform.translation = w.attack_offset.clone();
            c_sprite.rect = Some(w.attack_char_rect.clone());
            c_transform.translation = w.attack_char_offset.clone();
        } else {
            w_sprite.rect = Some(w.held_rect.clone());
            w_transform.translation = w.held_offset.clone();
            c_sprite.rect = Some(w.char_rect.clone());
            c_transform.translation = w.char_offset.clone();
        }

        if w.t > 0.0 {w.t -= dt; continue;}
        if w.ammo <= 0 {continue;}
        if !controller.shoot {continue;};
        w.ammo -= 1;
        // info!("Shooting!");
        w.t = w.cooldown;
    }
}

pub fn on_thrown_weapon_collision(
    event: On<CollisionStart>,
    state: Res<State<AppState>>,
    dropped: Query<&GlobalTransform, With<ThrownWeapon>>,
) {
    if state.get() != &STATE {return;}
    let Ok(d) = dropped.get(event.collider2) else {return;};
    // info!("Dropped weapon collision!");
}
