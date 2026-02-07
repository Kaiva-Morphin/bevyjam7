use crate::prelude::*;

use crate::core::plugin::CorePlugin;
pub mod character;
pub mod core;
pub mod tilemap;
pub mod prelude;
pub mod properties;

fn main() {
    App::new()
        .add_plugins(CorePlugin::default())
        .add_plugins(tilemap::plugin::MapPlugin)
        .run();
}
