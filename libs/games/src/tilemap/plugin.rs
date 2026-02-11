use crate::prelude::*;
use bevy_ecs_tiled::tiled::TiledPlugin;
#[cfg(not(target_arch = "wasm32"))]
use bevy_ecs_tiled::tiled::TiledPluginConfig;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        let mut conf = TiledPluginConfig{tiled_types_export_file: None, ..Default::default()};
        #[cfg(not(target_arch = "wasm32"))]
        let mut conf = TiledPluginConfig::default();
        
        conf.tiled_types_filter = TiledFilter::from(
            regex::RegexSet::new([
                r"^game::.*",
                r"^room::.*",
                r"^camera::.*",
                r"^bevy_sprite::text2d::Text2d$",
                r"^bevy_text::text::TextColor$",
                r"^.*::RigidBody$",
                r"^.*::CollisionLayers$",
                r"^.*::Sensor$",
                r"^.*::TilemapShadow$",
                r"^.*::TileShadow$",
                r"^.*::SpawnPoint$",
                r"^.*::StopTrigger$",
                r"^.*::NextTrigger$",
                r"^.*::CameraCenter$",
                r"^.*::SpawnPoint$",
                r"^.*::MiamiSpawnPoint$",
            ])
            .expect("Wrong regex"),
        );
        app
            .add_plugins((
                TiledPlugin(conf),
                TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            ))
            // .register_type::<SpawnPoint>()
            // .add_systems(Startup, spawn_map)
            // .add_observer(on_map_created)
            // .add_observer(on_object_spawned)
            // .add_observer(on_spawnpoint)
            // .add_observer(on_room_spawned)
            ;
    }
}

// fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(TiledMap(asset_server.load("maps/v0.1/map.tmx")))
//         ;
// }

// #[derive(Component, Default, Debug, Reflect)]
// #[reflect(Component, Default)]
// pub enum SpawnPoint {
//     #[default]
//     Unknown,
//     Player,
// }

// fn on_object_spawned(
//     collider_created: On<TiledEvent<ObjectCreated>>,
//     assets: Res<Assets<TiledMapAsset>>,
//     mut commands: Commands,
//     ch_q: Query<&Children>,
//     c_q: Query<&CollisionLayers>,
//     s_q: Query<&Sensor>,
//     r_q: Query<&RigidBody>,
// ) {
//     info!("A: {} B: {}", collider_created.event().origin, collider_created.event().entity);
//     let Ok(c) = ch_q.get(collider_created.event().origin) else {return;};
//     info!("C");
//     if let Ok(rb) = r_q.get(collider_created.event().origin) {
//         for child in c.iter() {
//             commands.entity(child).insert((Name::new("Extended"), rb.clone()));
//         }
//     }
//     if let Ok(s) = s_q.get(collider_created.event().entity) {
//         for child in c.iter() {
//             commands.entity(child).insert((Name::new("Extended"), s.clone()));
//         }
//     }
    
//     if let Ok(l) = c_q.get(collider_created.event().entity) {
//         for child in c.iter() {
//             commands.entity(child).insert((Name::new("Extended"), l.clone()));
//         }
//     }
// }



// fn on_spawnpoint(
//     add_spawn: On<Add, SpawnPoint>,
//     spawn_query: Query<(&SpawnPoint, &Transform)>,
//     mut cmd: Commands,
// ) {
//     let spawn_entity = add_spawn.event().entity;

//     let Ok((spawn_type, global_transform)) = spawn_query.get(spawn_entity) else {
//         return;
//     };

//     match spawn_type {
//         SpawnPoint::Player { .. } => {
//             // spawn_player(&mut cmd, Transform::from_translation(global_transform.translation()));
//             // spawn_player(&mut cmd, global_transform.clone());
//         }
//         _ => {}
//     };
// }


// fn _on_layer_spawned(
//     layer_created: On<TiledEvent<LayerCreated>>,
//     assets: Res<Assets<TiledMapAsset>>,
//     tiled_objects: Query<(&TiledObject, &Transform)>,
//     children: Query<&Children>,
//     mut cmd: Commands
// ) {
//     let Some(layer) = layer_created.event().get_layer(&assets) else {
//         return;
//     };
//     if layer.name == "Occluders" {
//         let Ok(lc) = children.get(layer_created.event().origin) else {return;};
//         for child in lc.iter() {
//             let Ok(tiled_object) = tiled_objects.get(child) else {continue;};
//             let (TiledObject::Rectangle { width, height }, t) = tiled_object else {continue;};
//             cmd.spawn((
//                 Name::new("Occluder"),
//                 Transform::from_translation(t.translation + vec3(*width / 2.0, -*height / 2.0, 0.0)),
//             ));
//             // cmd.entity(child).despawn();
//             if let Ok(c) = children.get(child) {
//                 for c in c.iter() {
//                     cmd.entity(c).despawn();
//                 }
//             }
//         }
//     }
// }


// fn _on_map_created(
//     _map_created: On<TiledEvent<MapCreated>>,
//     _map_query: Query<&TiledMapStorage, With<TiledMap>>,
//     // tiles_query: Query<(&TilePos, Option<&Biome>, Option<&Resource>)>,
// ) {
//     info!("Tilemap created!")
//     // Get the map entity and storage component
//     // let map_entity = map_created.event().origin;
//     // let Ok(map_storage) = map_query.get(map_entity) else {
//     //     return;
//     // };
//     // map_storage.get_layer(map, entity)

//     // We will iterate over all tiles from our map and try to access our custom properties
//     // for ((_tile_id, _tileset_id), entities_list) in map_storage.tiles() {
//     //     for tile_entity in entities_list {
//     //         let Ok((pos, biome, resource)) = tiles_query.get(*tile_entity) else {
//     //             continue;
//     //         };

//     //         // Here, we only print the content of our tile but we could also do some
//     //         // global initialization.
//     //         // A typical use case would be to initialize a resource so we can map a tile
//     //         // position to a biome and / or a resource (which could be useful for pathfinding)

//     //         if let Some(i) = biome {
//     //             // Only print the first tile to avoid flooding the console
//     //             info_once!("Found Biome [{:?} @ {:?}]", i, pos);
//     //         }

//     //         if let Some(i) = resource {
//     //             // Only print the first tile to avoid flooding the console
//     //             info_once!("Found Resource [{:?} @ {:?}]", i, pos);
//     //         }
//     //     }
//     // }
// }