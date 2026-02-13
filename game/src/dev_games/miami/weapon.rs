use std::f32::consts::FRAC_PI_2;

use avian2d::math::Vector;
use bevy::prelude::*;
use crate::prelude::AppState;
use rand::Rng;
use super::entity::{CharacterComponents, CharacterController, CharacterPivotPoint, CharacterSprite, Player};
use super::plugin::{BLOOD_Z_TRANSLATION, BODY_Z_TRANSLATION, THROWN_DAMAGE_MULTIPLIER, miami_dropped_weapon_layers, miami_pickup_weapon_layers, miami_projectile_damager_layer, miami_projectile_player_layer};
use super::shadows::ShadowCaster;
use crate::pathfinder::plugin::PathfinderObstacle;
use crate::prelude::*;
use super::{plugin::{MiamiAssets, STATE}, shadows::ShadowInit};



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

    pub damage: f32,
    pub ttl: f32,
    pub piercing: u32,

    pub projectile_speed: f32,

    pub throw_damage: f32,

    pub weapon_type: WeaponType
}
#[derive(Component)]
pub struct WeaponComponents {
    pub sprite: Entity,
}


#[derive(Component)]
pub struct ArmedCharacter(pub Entity);

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
                cooldown: 0.1,
                anim_time: 0.1,

                attack_char_rect: Rect::new(32.0, 0.0, 48.0, 48.0),
                attack_char_offset: vec3(0., -3., 0.),

                attack_offset: vec3(-2., -25., -0.05),
                attack_rect: Rect::new(16.0, 16.0, 32.0, 32.0),

                damage: 100.0,
                ttl: 5.0,
                piercing: 1,
                projectile_speed: 500.0,
                throw_damage: 100.0,

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
                cooldown: 0.1,
                anim_time: 0.1,

                attack_rect: Rect::new(32.0, 16.0, 64.0, 32.0),
                attack_offset: vec3(3., -10.0,  -0.3),
                
                attack_char_rect: Rect::new(0.0, 48.0, 32.0, 64.0),
                attack_char_offset: Vec3::ZERO,

                damage: 200.0,
                ttl: 0.01,
                piercing: 128,
                throw_damage: 50.0,

                ..Default::default()
            },
            WeaponType::Baguette => Weapon {
                weapon_type: WeaponType::Baguette,

                rect: Rect::new(0.0, 48.0, 32.0, 64.0),
                held_rect: Rect::new(0.0, 32.0, 32.0, 48.0),
                held_offset: vec3(-5., -6., -0.3),
                
                char_rect: Rect::new(0.0, 32.0, 32.0, 48.0),
                char_offset: Vec3::ZERO,

                ammo: u32::MAX,
                cooldown: 0.1,
                anim_time: 0.1,

                attack_rect: Rect::new(32.0, 32.0, 64.0, 48.0),
                attack_offset: vec3(3., -10.0,  -0.3),
                
                attack_char_rect: Rect::new(0.0, 48.0, 32.0, 64.0),
                attack_char_offset: Vec3::ZERO,

                damage: 300.0,
                piercing: 128,
                ttl: 0.01,
                
                throw_damage: 500.0,

                ..Default::default()
            },

            WeaponType::EnemyFists => Weapon {
                weapon_type: WeaponType::EnemyFists,

                rect: Rect::new(0.0, 0.0, 0.0, 0.0),
                held_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
                held_offset: Vec3::ZERO,
                
                char_rect: Rect::new(0.0, 16.0, 32.0, 32.0),
                char_offset: vec3(0., 0., 0.),

                ammo: u32::MAX,
                cooldown: 0.3,
                anim_time: 0.3,

                attack_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
                attack_offset: Vec3::ZERO,
                
                attack_char_rect: Rect::new(0.0, 64.0, 32.0, 96.0),
                attack_char_offset: vec3(0., -8., 0.),

                damage: 100.0,
                piercing: 128,
                ttl: 0.3,

                ..Default::default()
            }
        }
    }
}

impl Weapon {
    pub fn on_shoot(
        &self,
        cmd: &mut Commands,
        weapon: &Weapon,
        look_dir: Vec2,
        sprite: Entity,
        pos: Vec3,
        is_enemy: bool,
        assets: &Res<MiamiAssets>,
    ) -> Entity {
        let mut p = Projectile {
            damage: weapon.damage,
            lifetime: weapon.ttl,
            collided: Vec::new(),
            piercing: weapon.piercing,
            from_player: !is_enemy,
            despawn_on_wall: false,
        };
        let layer = if is_enemy {
            miami_projectile_player_layer()
        } else {
            miami_projectile_damager_layer()
        };
        match self.weapon_type {
            WeaponType::Pistol => {
                let mut t = Transform::from_translation(pos);
                t.rotation = Quat::from_rotation_z(look_dir.to_angle() - std::f32::consts::FRAC_PI_2);
                p.despawn_on_wall = true;
                cmd.spawn((
                    DespawnOnExit(STATE),
                    Name::new("Projectile"),
                    Sprite{
                        rect: Some(Rect::new(0., 0., 1., 16.)),
                        image: assets.projectiles.clone(),
                        ..Default::default()
                    },
                    Collider::circle(0.5),
                    LinearVelocity(look_dir * weapon.projectile_speed),
                    LinearDamping(0.0),
                    GravityScale(0.0),
                    CollisionEventsEnabled,
                    RigidBody::Dynamic,
                    layer,
                    Sensor,
                    t.clone(),
                    p
                )).id()
            },
            WeaponType::Axe | WeaponType::Baguette => {
                let t = Transform::from_xyz(0.0, -6.0, 0.0);
                let e = cmd.spawn((
                    DespawnOnExit(STATE),
                    layer,
                    t,
                    Sensor,
                    CollisionEventsEnabled,
                    Collider::capsule_endpoints(5.,Vector::new(-5.0, 0.0), Vector::new(3.0, 0.0)),
                    p
                )).id();
                
                cmd.entity(sprite).add_child(e);
                // cmd.spawn((
                // ));
                e
            }
            WeaponType::EnemyFists => {
                // let mut t = Transform::from_xyz(0.0, -6.0, 0.0);
                let e = cmd.spawn((
                    DespawnOnExit(STATE),
                    layer,
                    // t,
                    Sensor,
                    CollisionEventsEnabled,
                    Collider::circle(10.0),
                    // Collider::capsule_endpoints(5.,Vector::new(-5.0, 0.0), Vector::new(3.0, 0.0)),
                    p
                )).id();
                cmd.entity(sprite).add_child(e);
                e
            }
            // _ => unimplemented!()
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub lifetime: f32,
    pub collided: Vec<Entity>,
    pub piercing: u32,
    pub from_player: bool,
    pub despawn_on_wall: bool,
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
        DespawnOnExit(STATE),
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
        DespawnOnExit(STATE),
        Sensor,
        CollisionEventsEnabled,
        Collider::circle(8.0),
        Name::new("Weapon"),
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
pub struct WeaponOf(pub Entity);


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
    characters: Query<(&Children, &CharacterController, &CharacterComponents, &ArmedCharacter, Option<&Player>)>,
    mut weapons: Query<(&mut Weapon, &mut Transform, &WeaponComponents), Without<CharacterSprite>>,
    mut weapon_sprite: Query<(&mut Sprite, &GlobalTransform), (With<WeaponSprite>, Without<CharacterSprite>)>,
    mut char_sprite: Query<(&mut Sprite, &mut Transform), (With<CharacterSprite>, Without<WeaponSprite>, Without<WeaponComponents>)>,
    mut cmd: Commands,
    time: Res<Time>,
    assets: Res<MiamiAssets>,
){
    let dt = time.dt();
    for (_child, controller, cc, a, p) in characters.iter() {
        let Ok((mut w,mut w_transform, wc)) = weapons.get_mut(a.0) else {continue;};
        // info!("Weapon!");
        let Ok((mut w_sprite, sprite_transform)) = weapon_sprite.get_mut(wc.sprite) else {continue;};
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
        w.on_shoot(&mut cmd, &w, controller.look_dir, wc.sprite, sprite_transform.translation(), p.is_none(), &assets);
    }
}

pub fn on_thrown_weapon_collision(
    event: On<CollisionStart>,
    state: Res<State<AppState>>,
    mut dropped: Query<(&mut LinearVelocity, &GlobalTransform, &Weapon),  With<ThrownWeapon>>,
    mut character: Query<(Entity, &mut CharacterController, &GlobalTransform)>,
    mut cmd: Commands,
) {
    if state.get() != &STATE {return;}
    let Ok((mut lv, dt, w)) = dropped.get_mut(event.collider2) else {return;};
    let Ok((e, mut c, ct)) = character.get_mut(event.collider1) else {return;};
    let d = dt.translation() - ct.translation();
    let ld = if c.look_dir == Vec2::ZERO {
        Vec2::new(0.0, -1.0)
    } else {
        c.look_dir
    };
    c.last_impact_back = d.truncate().dot(ld) < 0.0;
    
    c.last_impact_dir = d.truncate();

    c.hp -= THROWN_DAMAGE_MULTIPLIER * lv.length() * w.throw_damage;
    info!("damage: {}", THROWN_DAMAGE_MULTIPLIER * lv.length() * w.throw_damage);
    lv.x = lv.x * 0.5;
    lv.y = lv.y * 0.5;
    if c.hp <= 0.0 {
        cmd.entity(e).remove::<RigidBody>();
    }
    info!("Dropped weapon collision!");
}


pub fn health_watcher(
    mut character: Query<(Entity, &GlobalTransform, &CharacterComponents, &mut CharacterController)>,
    sprite: Query<&GlobalTransform, With<CharacterSprite>>,
    mut cmd: Commands,
    assets: Res<MiamiAssets>,

) {
    let mut rng = rand::rng();
    for (e, t, components, mut controller) in character.iter_mut() {
        let dmg = controller.prev_hp - controller.hp;
        let Ok(_s) = sprite.get(components.sprite) else {continue;};
        if dmg == 0.0 {continue;}
        let mut b = Transform::from_translation(t.translation());
        b.translation.z += BLOOD_Z_TRANSLATION;
        let mut t = Transform::from_translation(t.translation());
        t.translation.z += BODY_Z_TRANSLATION;

        // t.rotation.z = controller.last_impact_dir.normalize_or_zero().to_angle();
        t.rotation = Quat::from_rotation_z(controller.last_impact_dir.normalize_or_zero().to_angle() + FRAC_PI_2);
        // t.translation.z += BODY_Z_TRANSLATION;
        // b.translation.z += BLOOD_Z_TRANSLATION;
        let up = (if controller.look_dir == Vec2::ZERO {Vec2::new(0.0, -1.0)} else {controller.look_dir}).normalize_or_zero();
        let right = Vec2::new(up.y, -up.x);
        
        let world_offset = right * controller.body_offset.x
                 + up    * controller.body_offset.y;
        t.translation.x += world_offset.x;
        t.translation.y += world_offset.y;
        // t.scale = Vec3::ONE;
        // b.scale = Vec3::ONE;
        info!("Dmg: {}", dmg);
        let idx;
        if dmg <= 20.0 {
            idx = 0;
        } else if dmg <= 40.0 {
            idx = 1;
        } else {
            idx = 2;
        }
        
        b.rotation = Quat::from_rotation_z(rng.random_range(0u32..=3u32) as f32 * FRAC_PI_2);
        // t.rotation.z = rng.random_range(0.0..std::f32::consts::PI * 2.0);
        let rect;
        if controller.last_impact_back {
            rect = controller.back_body_rect;
        } else {
            rect = controller.front_body_rect;
        };
        controller.prev_hp = controller.hp;
        if controller.hp <= 0.0 {
            // cmd.trigger()
            cmd.spawn((
                DespawnOnExit(STATE),
                t,
                Name::new("Body"),
                GlobalTransform::IDENTITY,
                Sprite {
                    image: controller.character.to_handle(&assets),
                    rect: Some(rect),
                    ..Default::default()
                }
            ));
            cmd.spawn((
                DespawnOnExit(STATE),
                b,
                Name::new("Blood"),
                GlobalTransform::IDENTITY,
                Sprite {
                    image: assets.decals.clone(),
                    color: controller.blood_color,
                    rect: Some(controller.blood_rects[2].clone()),
                    ..Default::default()
                }
            ));
            cmd.entity(e).despawn();
        } else {
            cmd.spawn((
                DespawnOnExit(STATE),
                b,
                Name::new("Blood"),
                Sprite {
                    image: assets.decals.clone(),
                    color: controller.blood_color,
                    rect: Some(controller.blood_rects[idx].clone()),
                    ..Default::default()
                }
            ));
        };
    }
}


pub fn on_projectile_hit(
    event: On<CollisionStart>,
    mut projectile: Query<(Entity, &mut Projectile, Option<&LinearVelocity>, &mut Transform)>,
    mut controllers : Query<(&mut CharacterController, &GlobalTransform, Option<&Player>)>,
    q : Query<(), With<PathfinderObstacle>>,
    state: Res<State<AppState>>,
    mut cmd: Commands,
){
    if state.get() != &STATE {return;};
    let Ok((e, mut projectile, vel, t)) = projectile.get_mut(event.collider1) else {return;};
    if let Ok(()) = q.get(event.collider2) && projectile.despawn_on_wall {
        cmd.entity(e).despawn();
        return;
    }
    let Ok((mut c, gt, _p)) = controllers.get_mut(event.collider2) else {return;};
    // if p.is_none() && !projectile.from_player {return;} 
    if projectile.piercing <= 0 {
        cmd.entity(e).despawn();
        return;
    };
    let d = gt.translation() - t.translation;
    let ld = if c.look_dir == Vec2::ZERO {
        Vec2::new(0.0, -1.0)
    } else {
        c.look_dir
    };
    let dir;
    if let Some(vel) = vel &&vel.length_squared() > 1.0 {
        dir = -vel.normalize();
    } else {
        dir = -d.truncate();
    }
    c.last_impact_back = dir.dot(ld) < 0.0;
    c.last_impact_dir = dir;
    projectile.piercing -= 1;
    info!("Hit: {} {}", projectile.damage, c.hp);
    c.hp -= projectile.damage;
}

pub fn update_projectile(
    mut projectile: Query<(&mut Projectile, Entity)>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    let dt = time.dt();
    for (mut projectile, e) in projectile.iter_mut() {
        projectile.lifetime -= dt;
        if projectile.lifetime <= 0.0 {
            cmd.entity(e).despawn();
            continue;
        }
    }
}

