use std::ops::{Deref, DerefMut};

use bevy::{
    app::{App, Startup, Update},
    asset::{Asset, Assets},
    core_pipeline::core_2d::{Camera2d, Camera2dBundle},
    ecs::{
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        ButtonState,
    },
    math::{primitives::Rectangle, vec3},
    reflect::TypePath,
    render::{color::Color, mesh::Mesh, render_resource::AsBindGroup},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    time::{Stopwatch, Time},
    transform::components::Transform,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<SdfCircle>::default()))
        .init_resource::<ZoomTimer>()
        .add_systems(Startup, setup)
        .add_systems(Update, (start_timer, tick_timer, zoom))
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
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        material: materials.add(SdfCircle {
            color: Color::rgb_linear(1., 1., 0.),
        }),
        ..Default::default()
    });
}

#[derive(Debug, Resource)]
struct ZoomTimer(Stopwatch);

impl Default for ZoomTimer {
    fn default() -> Self {
        let mut stopwatch = Stopwatch::default();
        stopwatch.pause();

        ZoomTimer(stopwatch)
    }
}

impl DerefMut for ZoomTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for ZoomTimer {
    type Target = Stopwatch;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn start_timer(mut keyboard_events: EventReader<KeyboardInput>, mut zoom_timer: ResMut<ZoomTimer>) {
    for event in keyboard_events.read() {
        match event.state {
            ButtonState::Released if event.key_code == KeyCode::KeyS => {
                zoom_timer.unpause();
            }
            _ => {}
        }
    }
}

fn tick_timer(time: Res<Time>, mut zoom_timer: ResMut<ZoomTimer>) {
    zoom_timer.tick(time.delta());
}

fn ease_out_circ(x: f32) -> f32 {
    f32::sqrt(1.0 - (x - 1.0).powf(2.0))
}

const SECONDS_TO_PERFECTION: f32 = 10.0;

fn zoom(mut zoom_timer: ResMut<ZoomTimer>, mut query: Query<&mut Transform, With<Camera2d>>) {
    if zoom_timer.elapsed_secs() >= SECONDS_TO_PERFECTION {
        zoom_timer.pause();
        return;
    }

    let Ok(mut camera_transform) = query.get_single_mut() else {
        return;
    };

    let scale = f32::max(
        -ease_out_circ(zoom_timer.elapsed_secs() / SECONDS_TO_PERFECTION) + 1.0,
        f32::MIN_POSITIVE,
    );
    camera_transform.scale = vec3(scale, scale, 1.0);
}
