use std::{
    io::{Cursor, Read},
    path::Path,
};

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
/// 
pub fn map_matcher(path: &Path) -> Box<dyn Read + 'static> {
    let path = path.to_str().expect("Invalid path").replace("\\", "/");
    let p = path.as_str();
    maps!(
        p =>
        "maps/v0.1/map.tmx"
        "maps/v0.1/tilemap.tsx"
    )
}
