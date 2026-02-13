use std::time::Duration;

use bevy::image::TextureAtlas;
use bevy_asset_loader::asset_collection::AssetCollection;
use games::prelude::DespawnOnExit;
use crate::prelude::*;




#[derive(AssetCollection, Resource)]
pub struct HintAssets {
    #[asset(path = "images/keys.png")]
    pub keys: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 57, tile_size_y = 42, columns = 1, rows = 3))]
    pub qad_atlas: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 57, tile_size_y = 42, columns = 1, rows = 3, offset_x = 57))]
    pub wasd_atlas: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 57, tile_size_y = 21, columns = 1, rows = 3, offset_x = 114))]
    pub space_atlas: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 19, tile_size_y = 21, columns = 1, rows = 3, offset_x = 171))]
    pub mouse_all_atlas: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 19, tile_size_y = 21, columns = 1, rows = 3, offset_x = 190))]
    pub mouse_rmb_atlas: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 19, tile_size_y = 21, columns = 1, rows = 3, offset_x = 209))]
    pub mouse_lmb_atlas: Handle<TextureAtlasLayout>,
}

pub enum KeyHint {
    KeysQAD,
    KeysSpace,
    KeysLmb,
    KeysRmb,
    KeysWASD,
    KeysMouseAll
}

impl KeyHint {
    pub fn get_asset(&self, assets: &HintAssets) -> Handle<TextureAtlasLayout> {
        match self {
            KeyHint::KeysQAD => assets.qad_atlas.clone(),
            KeyHint::KeysSpace => assets.space_atlas.clone(),
            KeyHint::KeysLmb => assets.mouse_lmb_atlas.clone(),
            KeyHint::KeysRmb => assets.mouse_rmb_atlas.clone(),
            KeyHint::KeysWASD => assets.wasd_atlas.clone(),
            KeyHint::KeysMouseAll => assets.mouse_all_atlas.clone(),
        }
    }
}


pub fn show_hints(
    cmd: &mut Commands,
    hints: Vec<KeyHint>,
    state: AppState,
    cam: Entity, // cam: Query<Entity, With<WorldCamera>>,
    assets: Res<HintAssets>,
) {
    let tween_in = Tween::new(
        EaseFunction::SineOut,
        Duration::from_secs_f32(HINT_IN_DURATION),
        UiTransformTranslationPxLens {
            start: vec2(0., -50.),
            end: vec2(0., 0.),
        }   
    );
    let tween_stay = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_secs_f32(HINT_STAY_DURATION),
        UiTransformTranslationPxLens {
            start: vec2(0., 0.),
            end: vec2(0., 0.),
        }
    );
    let tween_out = Tween::new(
        EaseFunction::SineIn,
        Duration::from_secs_f32(HINT_OUT_DURATION),
        UiTransformTranslationPxLens {
            start: vec2(0., 0.),
            end: vec2(0., -50.),
        }
    );
    let root = cmd.spawn((
        UiTargetCamera(cam),
        DespawnOnExit(state),
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            width: Val::Percent(100.),
            // height: Val::Percent(100.),
            column_gap: Val::Px(8.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Px(16.)),
            ..Default::default()
        },
        TweenAnim::new(tween_in.then(tween_stay).then(tween_out))
    )).id();
    let mut children = vec![];
    for key in hints {
        let c = cmd.spawn((
            ImageNode {
                texture_atlas: Some(TextureAtlas{layout: key.get_asset(&assets), index: 0}),
                image: assets.keys.clone(),
                ..Default::default()
            },
        )).id();
        children.push(c);
    }
    cmd.entity(root).insert(KeyHints { t: 0.0, hints: children.clone() }).add_children(&children);
}

#[derive(Component)]
pub struct KeyHints {
    t: f32,
    hints: Vec<Entity>,
}


pub fn update_hints(
    mut q: Query<(Entity, &mut KeyHints)>,
    time: Res<Time>,
    mut cmd: Commands,
    mut c_q: Query<&mut ImageNode>,
) {
    let dt = time.dt();
    for (e, mut hints) in q.iter_mut() {
        hints.t += dt;
        if hints.t > HINT_IN_DURATION + HINT_STAY_DURATION +  HINT_OUT_DURATION {
            cmd.entity(e).despawn();
        }
        for hint in &hints.hints {
            let Ok(mut c) = c_q.get_mut(*hint) else {continue;};
            let Some(t) = c.texture_atlas.as_mut() else {continue;};
            t.index = (hints.t * HINT_ANIM_SPEED) as usize % 3;
        }
    }
}

pub const HINT_ANIM_SPEED : f32 = 3.0;
pub const HINT_IN_DURATION : f32 = 0.3;
pub const HINT_STAY_DURATION : f32 = 10.0;
pub const HINT_OUT_DURATION : f32 = 0.3;