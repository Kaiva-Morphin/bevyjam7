use bevy::prelude::*;
use games::prelude::{AppState, LastScreenshot};
use properties::{HighresCamera, WorldCamera};

use crate::dev_games::fnaf::plugin::FNAFAssets;

const STATE: AppState = AppState::Novel2Fnaf;
const NEXT_STATE: AppState = AppState::Fnaf;

pub struct Novel2FnafPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = STATE)]
#[states(scoped_entities)]
enum LocalState {
    #[default]
    Initial,
    Zooming,
    Finished,
}

impl Plugin for Novel2FnafPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_sub_state::<LocalState>()
            .add_systems(OnEnter(STATE), setup)
            .add_systems(Update, zoom.run_if(in_state(LocalState::Zooming)))
            .add_systems(Update, cleanup.run_if(in_state(LocalState::Finished)))
            // .add_systems(Update, (t, update_mouse_pos).run_if(in_state(STATE)))
            // .add_observer(collision_handler)
            ;
    }
}

const RECT_BL: Vec2 = Vec2::new(-495.60846, -43.876827);
const RECT_TR: Vec2 = Vec2::new(-22.935652, 217.38959);

#[derive(Component)]
struct NovelScreenshot;

#[derive(Resource)]
struct FnafRect(Rect);

fn setup(
    mut cmd: Commands,
    mut localstate: ResMut<NextState<LocalState>>,
    last: Res<LastScreenshot>,
    cam: Query<Entity, With<HighresCamera>>,
    mut camera_transform_q: Query<&mut Projection, With<WorldCamera>>,
    fnaf_assets: Res<FNAFAssets>,
) {
    if let Some(image) = last.image.clone() {
        let fnaf_rect = Rect::from_corners(RECT_BL, RECT_TR);
        localstate.set(LocalState::Zooming);
        let cam = cam.iter().next().expect("No cam!");
        cmd.spawn((
            Name::new("Screenshot"),
            DespawnOnEnter(NEXT_STATE),
            UiTargetCamera(cam),
            Sprite {
                image,
                ..default()
            },
            NovelScreenshot,
        ));

        cmd.spawn((
            Name::new("Fnaf room"),
            DespawnOnEnter(NEXT_STATE),
            UiTargetCamera(cam),
            Sprite {
                image: fnaf_assets.room.clone(),
                ..default()
            },
            Transform {
                translation: fnaf_rect.center().extend(0.0),
                scale: Vec3::new(0.24, 0.25, 0.0),
                ..default()
            },
            NovelScreenshot,
        ));
        cmd.insert_resource(FnafRect(fnaf_rect));
        if let Projection::Orthographic(proj) = &mut *camera_transform_q.single_mut().unwrap() {
            proj.scale = 2.77;
        }
    }
}


fn zoom(
    mut camera_transform_q: Query<(&mut Projection, &mut Transform), With<WorldCamera>>,
    window_q: Query<&Window>,
    mut localstate: ResMut<NextState<LocalState>>,
    fnaf_rect: Res<FnafRect>,
) {
    const TRANSLATION_PF: f32 = 1.;
    const SCALE_PF: f32 = 0.003;
    const THRESHOLD: f32 = 0.4;

    if let Ok(window) = window_q.single() {
        let window_size = Vec2::new(window.width(), window.height());
        let (mut projection, mut transform) = camera_transform_q.single_mut().unwrap();
        let direction_2d = (fnaf_rect.0.center() - transform.translation.xy()).normalize_or_zero();
        transform.translation.x += direction_2d.x * TRANSLATION_PF;
        transform.translation.y += direction_2d.y * TRANSLATION_PF;
        
        let target_scale = ((fnaf_rect.0.size().x / window_size.x).max(fnaf_rect.0.size().y / window_size.y)).abs();
        if let Projection::Orthographic(proj) = &mut *projection {
            // println!("{}", proj.scale - target_scale);
            proj.scale += (target_scale - proj.scale) * SCALE_PF;
            
            if (proj.scale - target_scale).abs() < THRESHOLD {
                localstate.set(LocalState::Finished);
            }
        }
    }
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<(&mut Transform, &mut Projection), With<WorldCamera>>,
    mut appstate: ResMut<NextState<AppState>>,
) {
    cmd.remove_resource::<FnafRect>();

    let (mut transform, mut projection) = cam.single_mut().unwrap();
    transform.translation = Vec3::ZERO;
    appstate.set(NEXT_STATE);
    if let Projection::Orthographic(proj) = &mut *projection {
        proj.scale = 0.7;
    }
}