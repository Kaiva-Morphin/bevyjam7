use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiPlugin, EguiPreUpdateSet};
use bevy_tweening::TweeningPlugin;
use camera::CameraPlugin;
use debug_utils::{
    avian::plugin::SwitchableAvianDebugPlugin, debug_overlay::DebugOverlayPlugin,
    inspector::plugin::SwitchableEguiInspectorPlugin,
};
use room::RoomPlugin;
use crate::global_music::plugin::GlobalMusicPlugin;

use crate::{shaders::bg::BGPlugin, tilemap::plugin::MapPlugin};

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Game".to_string(),
                            canvas: Some("#bevy".to_owned()),
                            fit_canvas_to_parent: true,
                            prevent_default_event_handling: true,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(AssetPlugin {
                        meta_check: bevy::asset::AssetMetaCheck::Never,
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
                PhysicsPlugins::default(),
                // PhysicsPlugins::default(),
                EguiPlugin::default(),
                CameraPlugin{initial_target_zoom: 1.0},
                RoomPlugin::uninited(),
                GlobalMusicPlugin,
                // CharacterPlugin,
                SwitchableEguiInspectorPlugin::default(),
                DebugOverlayPlugin::default(),
                SwitchableAvianDebugPlugin::enabled(),
                MapPlugin,
                TweeningPlugin,
                BGPlugin,
                // GameStatesPlugin,
                // PixelCameraPlugin,
                // CameraControllerPlugin,
                // bevy_framepace::FramepacePlugin,
            ))
            // .insert_resource(bevy_framepace::FramepaceSettings{limiter: bevy_framepace::Limiter::from_framerate(60.0)})
            .add_systems(PreUpdate, super::egui_font::init_egui_font.after(EguiPreUpdateSet::InitContexts).run_if(run_once))
            // .add_systems(PreStartup, 
            //     debug_ui_to_camera
            //     .after(pixel_utils::camera::setup_camera)
            //     .after(debug_utils::debug_overlay::init))
            ;
    }
}
