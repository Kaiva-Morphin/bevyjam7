use crate::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TilemapShadow;

pub fn setup_tilemap_shadows(
    layer_created: On<TiledEvent<LayerCreated>>,
    mut tile_shadow: Query<&mut Transform, With<TilemapShadow>>,
    state: Res<State<AppState>>,
){
    if state.get() != &super::plugin::STATE {return;}
    let e = layer_created.origin;
    let Ok(mut t) = tile_shadow.get_mut(e) else {return;};
    t.translation.x += MIAMI_SHADOW_OFFSET.x;
    t.translation.y += MIAMI_SHADOW_OFFSET.y;
}
