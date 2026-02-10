use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;




#[derive(AssetCollection, Resource)]
pub struct MiamiAssets {
    #[asset(path = "maps/platformer/map.tmx")]
    tilemap: Handle<TiledMapAsset>,
    #[asset(path = "maps/platformer/character.png")]
    character: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    character_layout: Handle<TextureAtlasLayout>,
}
