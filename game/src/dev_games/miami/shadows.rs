use crate::{dev_games::miami::{entity::CharacterPivotPoint, plugin::STATE}, prelude::*};


#[derive(Component)]
pub struct ShadowOf(pub Entity);


#[derive(Component)]
pub struct ShadowInit;


#[derive(Component)]
pub struct ShadowCaster;

#[derive(Component)]
pub struct ShadowPivot;


pub fn setup_shadows(
    mut cmd: Commands,
    q: Query<(Entity, &ChildOf, &Sprite), With<ShadowInit>>
) {
    for (e, c, s)  in q.iter() {
        let mut s = s.clone();
        s.color = miami_shadow_color();
        let shadow = cmd.spawn((
            DespawnOnExit(STATE),
            Name::new("Shadow"),
            s,
            Transform::from_translation(MIAMI_SHADOW_OFFSET),
            ShadowOf(e),
        )).id();
        cmd.entity(e).remove::<ShadowInit>()
            .insert((ShadowCaster, Name::new("ShadowCaster")));
        ;
        // cmd.entity(c.parent()).add_child(shadow);
    }
}


pub fn update_shadows(
    shadow_q: Query<(&mut Sprite, &mut Transform, &ShadowOf), (With<ShadowOf>, Without<ShadowCaster>)>,
    caster_q: Query<(&Sprite, &Transform, &GlobalTransform), (With<ShadowCaster>, Without<ShadowOf>)>,
){
    for (mut sprite, mut transform, shadow) in shadow_q {
        let Ok((s, t, g)) = caster_q.get(shadow.0) else {continue};
        transform.translation = g.translation() + MIAMI_SHADOW_OFFSET;
        transform.rotation = g.rotation();
        sprite.rect = s.rect.clone();
    }
}



// pub fn update_shadows(
//     mut shadow_q: Query<(&mut Sprite, &mut Transform, &ShadowOf), Without<ShadowCaster>>,
//     caster_q: Query<(&Sprite, &GlobalTransform), With<ShadowCaster>>,
// ) {
//     for (mut sprite, mut transform, shadow) in shadow_q {
//         let Ok((caster_sprite, caster_global)) = caster_q.get(shadow.0) else {
//             continue;
//         };

//         // let local_offset =
//         //     caster_global.rotation().inverse() * MIAMI_SHADOW_OFFSET;

//         let local_offset =
//             caster_global.rotation() * MIAMI_SHADOW_OFFSET;

//         transform.translation = local_offset;
//         sprite.rect = caster_sprite.rect.clone();
//     }
// }



pub fn cleanup_shadows(
    mut cmd: Commands,
    shadows: Query<(Entity, &ShadowOf)>,
    casters: Query<Entity, With<ShadowCaster>>
) {
    for (e, shadow) in shadows {
        let Err(_) = casters.get(shadow.0) else {continue;};
        cmd.entity(e).despawn();
    }
}
