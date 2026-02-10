use bevy::{render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::{Material2d, Material2dPlugin}, window::WindowResized};

use crate::prelude::*;

const SHADER_ASSET_PATH: &str = "shaders/bg.wgsl";


pub struct BGPlugin;

impl Plugin for BGPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, setup)
            .add_plugins(
                Material2dPlugin::<BgMaterial>::default()
            )
            .add_systems(PostStartup, setup)
            .add_systems(Update, window_resize)
            ;
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BgMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(BgMaterial {size: Vec4::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32, 0., 0.)})),
        Transform::default().with_scale(Vec3::splat(128.)),
        HIGHRES_LAYERS
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct BgMaterial {
    #[uniform(0)]
    size: Vec4,
}

impl Material2d for BgMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

fn window_resize(
    mut r: MessageReader<WindowResized>,
    mut query: Query<(&MeshMaterial2d<BgMaterial>, &mut Transform)>,
    mut materials: ResMut<Assets<BgMaterial>>,
) {
    let Some(e) = r.read().last() else {return;};
    if e.width == 0.0 || e.height == 0.0 {return;}
    for (material, mut transform) in query.iter_mut() {
        let material = materials.get_mut(material).unwrap();
        material.size = Vec4::new(e.width as f32, e.height as f32, 0., 0.);
        transform.scale = vec3(e.width as f32, e.height as f32, 1.);
    }
}
