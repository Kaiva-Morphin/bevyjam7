use std::collections::VecDeque;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use properties::*;

#[derive(Default)]
pub struct RoomPlugin {
    uninited: bool
}

impl RoomPlugin {
    pub fn uninited() -> Self {
        Self { uninited: true }
    }
}


impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<MapRoom>()
            .add_observer(on_room_entered)
            .add_observer(on_room_exited)
            ;
        if !self.uninited {
            app
                .insert_resource(RoomController::default());
        }
    }
}

#[derive(Component)]
pub struct Focusable;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct MapRoom {
    pub zoom: f32,
}


#[derive(Component, Clone, Default)]
pub struct RoomBounds {
    pub zoom: f32,
    pub ld: Vec3,
    pub ru: Vec3,
}


#[derive(Component)]
pub struct EnteredRoom {
    pub entity: Entity,
    pub room: RoomBounds
}

#[derive(Resource, Default)]
pub struct RoomController {
    // pub dynamic_rooms: Vec<Entity>,
    pub rooms: VecDeque<EnteredRoom>,
}



pub fn on_room_spawned(
    collider_created: On<TiledEvent<ColliderCreated>>,
    spawn_query: Query<&MapRoom>,
    tiled_objects: Query<&TiledObject>,
    parents: Query<(&GlobalTransform, &ChildOf)>,
    mut cmd: Commands,
) {
    let spawn_entity = collider_created.event().origin;
    let Ok((gt, p)) = parents.get(spawn_entity) else {return;};
    let Ok(map_room) = spawn_query.get(p.parent()) else {return;};
    let Ok(room) = tiled_objects.get(p.parent()) else {return;};
    let TiledObject::Rectangle { width, height } = room else {return;};
    let t = gt.translation();
    let ld = t - vec3(0.0, *height, 0.0);
    let ru = t + vec3(*width, 0.0, 0.0);
    cmd.entity(spawn_entity).insert((
        Name::new("Room"),
        RigidBody::Static,
        Sensor,
        room_layers(),
        RoomBounds {
            zoom: map_room.zoom,
            ld,
            ru
        },
        CollisionEventsEnabled,
    ))
        
        ;
}

fn on_room_entered(
    event: On<CollisionStart>,
    controller: Option<ResMut<RoomController>>,
    rooms: Query<(&RoomBounds, Entity)>,
    player: Query<Entity, With<Focusable>>
) {
    let Some(mut controller) = controller else {return;};
    let other_entity = event.collider2;
    let room_entity = event.collider1;
    let Ok((room, e)) = rooms.get(room_entity) else {
        return;
    };
    if player.contains(other_entity) {
        controller.rooms.push_front(EnteredRoom { entity: e, room: room.clone() });
    }
}

fn on_room_exited(
    event: On<CollisionEnd>,
    controller: Option<ResMut<RoomController>>,
    player: Query<Entity, With<Focusable>>
) {
    let Some(mut controller) = controller else {return;};
    let room_entity = event.collider1;
    let other_entity = event.collider2;
    if player.contains(other_entity) {
        controller.rooms.retain(|e| e.entity != room_entity);
    }
}