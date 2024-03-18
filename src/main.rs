use bevy::{
    app::{App, Startup, Update},
    asset::{Asset, Assets},
    core_pipeline::core_2d::{Camera2d, Camera2dBundle},
    ecs::{
        query::With,
        system::{Commands, Local, Query, ResMut, Resource},
    },
    math::{primitives::Rectangle, vec3, Vec2},
    reflect::TypePath,
    render::{color::Color, mesh::Mesh, render_resource::AsBindGroup},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self, Slider},
    EguiContexts, EguiPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<SdfCircle>::default()))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, render_ui)
        .run();
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
struct SdfCircle {
    #[uniform(0)]
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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(vec3(1.0, 1.0, 1.0)),
        ..Default::default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        material: materials.add(SdfCircle {
            color: Color::rgb_linear(1., 1., 0.),
        }),
        ..Default::default()
    });
}

#[derive(Resource)]
struct UiState {
    zoom_level: f32,
}

impl Default for UiState {
    fn default() -> Self {
        UiState { zoom_level: 1.0 }
    }
}

fn render_ui(
    mut state: Local<UiState>,
    mut contexts: EguiContexts,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    egui::Window::new("CIRCLE").show(contexts.ctx_mut(), |ui| {
        ui.add(Slider::new(&mut state.zoom_level, 1.0..=100.0).text("ZOOOOM!"));
    });

    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    transform.scale = Vec2::splat(1.0 / state.zoom_level).extend(1.0);
}
