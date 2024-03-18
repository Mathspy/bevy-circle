use bevy::{
    app::{App, Startup},
    asset::{Asset, Assets},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, ResMut},
    math::primitives::Rectangle,
    reflect::TypePath,
    render::{color::Color, mesh::Mesh, render_resource::AsBindGroup},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<SdfCircle>::default()))
        .add_systems(Startup, setup)
        .run();
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
struct SdfCircle {
    #[uniform(0)]
    radius: f32,
    #[uniform(1)]
    color: Color,
}

impl Material2d for SdfCircle {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/sdf_circle.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SdfCircle>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        material: materials.add(SdfCircle {
            radius: 50.0,
            color: Color::rgb_linear(1., 1., 0.),
        }),
        ..Default::default()
    });
}
