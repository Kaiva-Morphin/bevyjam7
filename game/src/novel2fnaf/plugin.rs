use bevy::prelude::*;
use games::prelude::{AppState, LastScreenshot};
use properties::WorldCamera;

const STATE: AppState = AppState::Novel2Fnaf;
const NEXT_STATE: AppState = AppState::Novel;

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
            // .add_observer(collision_handler)
            ;
    }
}

// Rect to zoom to: center (1230.5, 804.5), size 913x507
const ZOOM_RECT_CENTER: Vec2 = Vec2::new(1230.5, 804.5);
const ZOOM_RECT_SIZE: Vec2 = Vec2::new(913.0, 507.0);

fn setup(
    mut cmd: Commands,
    mut localstate: ResMut<NextState<LocalState>>,
    last: Res<LastScreenshot>,
) {
    if let Some(image) = last.image.clone() {
        localstate.set(LocalState::Zooming);
        cmd.spawn((
            DespawnOnExit(STATE),
            Sprite {
                image,
                ..default()
            }
        ));
    }
}

fn zoom(
    mut camera_transform_q: Query<(&mut Projection, &mut Transform), With<WorldCamera>>,
    window_q: Query<&Window>,
    mut localstate: ResMut<NextState<LocalState>>,
) {
    if let Ok(window) = window_q.single() {
        let window_size = Vec2::new(window.width(), window.height());
        
        let (mut projection, mut transform) = camera_transform_q.single_mut().unwrap();
        
        // Position camera at rect center
        transform.translation = ZOOM_RECT_CENTER.extend(transform.translation.z);
        
        // Calculate scale to fit the rect in viewport with some padding
        // scale = max(rect_width / window_width, rect_height / window_height)
        let scale = (ZOOM_RECT_SIZE.x / window_size.x).max(ZOOM_RECT_SIZE.y / window_size.y);
        
        if let Projection::Orthographic(proj) = &mut *projection {
            proj.scale = scale;
        }
        
        localstate.set(LocalState::Finished);
    }
}

fn cleanup(
    mut cmd: Commands,
    mut cam: Query<&mut Transform, With<WorldCamera>>,
) {
    // cam.iter_mut().next().expect("No cam!").translation = Vec3::ZERO;
}