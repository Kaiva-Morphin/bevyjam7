use std::{
    io::{Cursor, Read},
    path::Path,
};

use bevy::log::info;

#[cfg(feature = "wasm")]
macro_rules! maps {
    ($path:ident => $($map_path:expr)*$(;)?) => {
        match $path {
            $(
                $map_path => Box::new(Cursor::new(include_bytes!(concat!("../../../../game/assets/", $map_path)))),
            )*
            _ => panic!("No map rule exists! Please add map to static registry.")
        }
    };
}

#[cfg(feature = "wasm")]
pub fn map_matcher(path: &Path) -> Box<dyn Read + 'static> {
    let path = path.to_str().expect("Invalid path").replace("\\", "/");
    let p = path.as_str();
    info!("Loading map: {}", p);
    maps!(
        p =>
        "maps/v0.1/map.tmx"
        "maps/v0.1/tilemap.tsx"
        "maps/platformer/map.tmx"
        "maps/platformer/tilemap.tsx"
        "maps/GD/pacman.tmx"
        "maps/GD/GD tiles.tsx"
        "maps/miami/map.tmx"
        "maps/miami/miami.tsx"
    )
}
