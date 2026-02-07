pub mod plugin {
    use avian2d::prelude::*;
    use bevy::prelude::*;
    // use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

    #[derive(Default)]
    pub struct SwitchableAvianDebugPlugin(pub bool);
    impl SwitchableAvianDebugPlugin {
        pub fn enabled() -> Self {
            Self(true)
        }
        pub fn disabled() -> Self {
            Self(false)
        }
    }
    impl Plugin for SwitchableAvianDebugPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_plugins(PhysicsDebugPlugin::default())
                .add_systems(Update, update);
        }
    }

    fn update(
        // mut cmd: Commands,
        k: Res<ButtonInput<KeyCode>>,
        mut store: ResMut<GizmoConfigStore>,
    ) {
        if k.just_pressed(KeyCode::F2) {
            let (gc, _) = store.config_mut::<PhysicsGizmos>();
            gc.enabled = !gc.enabled;
        }
    }
}
