use std::collections::VecDeque;

use bevy::{camera::{ImageRenderTarget, RenderTarget, ScalingMode}, input::mouse::MouseWheel, prelude::*, render::{render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::Hdr}, window::WindowResized};
use kaiv_utils::prelude::ExpDecay;
use properties::*;
use room::*;



pub struct CameraPlugin{initial_target_zoom: f32}
impl Default for CameraPlugin {
    fn default() -> Self {
        Self{initial_target_zoom: 0.5}
    }
}


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .insert_resource(CameraController{target_zoom: self.initial_target_zoom, ..default()})
            .add_systems(Update, (window_resize, tick_camera))
            .add_observer(focus_player)
            ;
    }
}


#[derive(Resource)]
pub struct ViewportCanvas {
    pub image: Handle<Image>,
}

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    
    let canvas_size = Extent3d {
        width: TARGET_WIDTH,
        height: TARGET_HEIGHT,
        ..default()
    };
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    canvas.resize(canvas_size);


    let image_handle = images.add(canvas);
    commands.spawn((
        Name::new("WorldCamera"),
        Camera2d,
        WorldCamera,
        Msaa::Off,
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        RenderTarget::Image(ImageRenderTarget{ handle: image_handle.clone(), scale_factor: 1.0 }),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: TARGET_WIDTH as f32, 
                min_height: TARGET_HEIGHT as f32,
            },
            scale: 0.8,
            // scale: 1.0,
            near: -1000.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        }),
        WORLD_LAYERS,
    ));
    commands.insert_resource(
        ViewportCanvas {
            image: image_handle.clone()
        }
    );
    commands.spawn((
        Name::new("HighresCamera"),
        HIGHRES_LAYERS,
        Camera2d,
        Msaa::Off,
        Camera {
            order: 1,
            is_active: true,
            clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.1, 0.0, 0.1)),
            ..Default::default()
        }
    ));
    commands.spawn((Sprite::from_image(image_handle), HIGHRES_LAYERS));
}

fn window_resize(
    mut r: MessageReader<WindowResized>,
    mut images: ResMut<Assets<Image>>,
    canvas: Res<ViewportCanvas>,
) {
    let Some(e) = r.read().last() else {return;};
    let Some(img) = images.get_mut(&canvas.image) else {return;};
    let target_aspect = TARGET_WIDTH as f32 / TARGET_HEIGHT as f32;
    let w = (e.width as f32).max(1.0);
    let h = (e.height as f32).max(1.0);
    let aspect = w / h;
    if target_aspect > aspect {
        img.resize(Extent3d{width: e.width as u32, height: (e.width as f32 / target_aspect) as u32, ..Default::default()});
    } else {
        img.resize(Extent3d{width: (e.height as f32 * target_aspect) as u32, height: e.height as u32, ..Default::default()});
    }
}


#[derive(Debug, Default)]
pub enum CameraMode {
    #[default]
    Follow,
    Free,
}

impl CameraMode {
    fn opposite(&self) -> Self {
        match self {
            Self::Follow => Self::Free,
            Self::Free => Self::Follow,
        }
    }
}


#[derive(Resource)]
pub struct CameraController {
    focused_entities: VecDeque<Entity>,
    camera_mode: CameraMode,
    target_zoom: f32,
    follow_speed: f32,
    zoom_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            focused_entities: VecDeque::new(),
            camera_mode: CameraMode::default(),
            target_zoom: 1.0,
            follow_speed: 0.7,
            zoom_speed: 0.9
        }
    }
}

// todo!: initial room
fn focus_player(
    player: On<Add, Player>,
    pq: Query<&GlobalTransform, (With<Player>, Without<WorldCamera>)>,
    mut cq: Query<(Entity, &mut Projection), (With<WorldCamera>, Without<Player>)>,
    mut cmd: Commands,
    mut camera_controller: ResMut<CameraController>,
) {
    camera_controller.focused_entities.push_front(player.entity);
    let Ok(pt) = pq.get(player.entity) else {return;};
    let Some((ce, mut p)) = cq.iter_mut().next() else {return;}; 
    let Projection::Orthographic(p) = &mut *p else {warn!("Camera without perspective projection"); return;};
    p.scale = camera_controller.target_zoom;
    cmd.entity(ce).insert(Transform::from_translation(pt.translation()));
}


fn tick_camera(
    time: Res<Time>,
    mut camera: Query<(&Camera, &mut Projection, &mut Transform, &GlobalTransform), With<WorldCamera>>,
    mut camera_controller: ResMut<CameraController>,   
    room_controller: Option<Res<RoomController>>,
    targets: Query<&Transform, Without<WorldCamera>>,
    window: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
    mut freecam: Local<(f32, Vec3)>,
    mut mouse: MessageReader<MouseWheel>,
) {
    let dt = time.delta_secs().max(MAX_DT);
    let Some(entity) = camera_controller.focused_entities.front() else {return;};
    let Ok(cam_target) = targets.get(*entity) else {warn!("Target without transform in cam focus"); return;};
    let Ok((cam, mut p, mut t, gt)) = camera.single_mut() else {warn!("Camera without transform"); return;};
    let Projection::Orthographic(p) = &mut *p else {warn!("Camera without perspective projection"); return;};
    let Some(window) = window.iter().next() else {return;};
    let mut target = cam_target.translation;
    let mut z = 0.0;
    for ev in mouse.read() {
        z -= ev.y;
    }
    let mut target_zoom = camera_controller.target_zoom + z * 0.05;
    let mut zoom_speed = camera_controller.zoom_speed;
    let mut follow_speed = camera_controller.follow_speed;
    'a : {
        'b : {
            let (s, t) = &mut *freecam;
            if keys.just_pressed(KeyCode::ControlRight) {
                *t = cam_target.translation;
                *s = p.scale;
                camera_controller.camera_mode = camera_controller.camera_mode.opposite();
            }
            let CameraMode::Free = camera_controller.camera_mode else {break 'b;};
            let mut v = Vec3::ZERO;
            if keys.pressed(KeyCode::ArrowUp) { v.y += 1.0; }
            if keys.pressed(KeyCode::ArrowDown) { v.y -= 1.0; }
            if keys.pressed(KeyCode::ArrowLeft) { v.x -= 1.0; }
            if keys.pressed(KeyCode::ArrowRight) { v.x += 1.0; }
            if keys.pressed(KeyCode::ShiftRight) { v *= 500.; } else { v *= 100.; }
            *t += v * dt;
            *s += z * 0.05;
            target = t.clone();
            target_zoom = *s;
            zoom_speed = 1.0;
            follow_speed = 1.0;
            break 'a;
        }
        let Some(room_controller) = room_controller else {break 'a;};
        let Some(EnteredRoom{room: RoomBounds{ld, ru, ..}, ..}) = room_controller.rooms.front() else {break 'a;};
        let Some((sld, sru)) = screen_rect_in_world(cam, &gt, window) else {break 'a;};
        let sld = sld.extend(0.0);
        let sru = sru.extend(0.0);
        let sd = sru - sld;
        let d = ru - ld;
        let sx = sd.x;
        let sy = sd.y;
        if sx > d.x {
            target.x = (ld.x + ru.x) * 0.5
        }  else {
            if target.x - sx * 0.5 < ld.x {target.x = ld.x + sx * 0.5}
            if target.x + sx * 0.5 > ru.x {target.x = ru.x - sx * 0.5}
        }
        if sy > d.y {
            target.y = (ld.y + ru.y) * 0.5
        } else {
            if target.y - sy * 0.5 < ld.y {target.y = ld.y + sy * 0.5}
            if target.y + sy * 0.5 > ru.y {target.y = ru.y - sy * 0.5}
        }
    }
    
    t.translation = t.translation.exp_decay(target, follow_speed, dt);
    p.scale = p.scale.exp_decay(target_zoom, zoom_speed, dt);
}

fn screen_rect_in_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
) -> Option<(Vec2, Vec2)> {
    let min = Vec2::new(0.0, window.height());
    let max = Vec2::new(window.width(), 0.0);
    let world_min = camera.viewport_to_world_2d(camera_transform, min).ok()?;
    let world_max = camera.viewport_to_world_2d(camera_transform, max).ok()?;
    Some((world_min, world_max))
}