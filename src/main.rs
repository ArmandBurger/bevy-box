//! The Box
//!
//! A box with balls inside it.
//! Rotating ever so slowly.
//!
//! Oh, no! The box has sprung a leak.

use bevy::{
    app::PluginGroupBuilder,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    log::{Level, LogPlugin},
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
    sprite::{Wireframe2dConfig, Wireframe2dPlugin},
    window::WindowMode,
};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(BevyBoxPlugin::bevy_plugins())
        .add_systems(Startup, BevyBoxPlugin::setup)
        .add_systems(
            Update,
            (BevyBoxPlugin::toggle_wireframe, BevyBoxPlugin::rotate_box),
        )
        .run();
}

pub struct BevyBoxPlugin;

impl BevyBoxPlugin {
    fn window_plugin() -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                name: Some("Bevy Box".into()),
                title: "Bevy Box".into(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 800.0,
                    min_height: 600.0,
                    ..Default::default()
                },
                // Biiiig but not fullscreen.
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn log_plugin() -> LogPlugin {
        LogPlugin {
            level: Level::INFO,
            filter: "goon=trace,wgpu_hal=warn".into(),
            ..Default::default()
        }
    }

    fn fps_overlay() -> FpsOverlayPlugin {
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_color: Color::srgb_u8(0, u8::MAX, 0),
                ..Default::default()
            },
        }
    }

    fn default_plugins() -> PluginGroupBuilder {
        DefaultPlugins
            .set(Self::window_plugin())
            .set(Self::log_plugin())
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..Default::default()
                }),
                ..Default::default()
            })
    }

    fn bevy_plugins() -> (
        PluginGroupBuilder,
        FpsOverlayPlugin,
        Wireframe2dPlugin,
        RapierPhysicsPlugin,
        // RapierDebugRenderPlugin, // Note: only chaos ensues.
        PanCamPlugin,
    ) {
        (
            Self::default_plugins(),
            Self::fps_overlay(),
            Wireframe2dPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            PanCamPlugin::default(),
        )
    }
}

#[derive(Component)]
#[require(Transform)]
struct Box;

impl BevyBoxPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let mut ortho = OrthographicProjection::default_2d();
        // Adjust this value to zoom in or out. Smaller number zooms in, larger number zooms out.
        ortho.scale = 0.2;

        commands.spawn((
            Camera2d::default(),
            Projection::Orthographic(ortho),
            PanCam::default(),
        ));

        commands
            .spawn((Box, InheritedVisibility::VISIBLE))
            .with_children(|spawner| {
                spawner.spawn((
                    Mesh2d(meshes.add(Rectangle::new(2.0, 100.0))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(1.0, 50.0),
                    Transform::from_xyz(-49.0, 0.0, 0.0),
                ));
                spawner.spawn((
                    Mesh2d(meshes.add(Rectangle::new(2.0, 100.0))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(1.0, 50.0),
                    Transform::from_xyz(49.0, 0.0, 0.0),
                ));
                spawner.spawn((
                    Mesh2d(meshes.add(Rectangle::new(100.0, 2.0))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(50.0, 1.0),
                    Transform::from_xyz(0.0, -49.0, 0.0),
                ));
                spawner.spawn((
                    Mesh2d(meshes.add(Rectangle::new(100.0, 2.0))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(50.0, 1.0),
                    Transform::from_xyz(0.0, 49.0, 0.0),
                ));
            });

        // Balls are not children of the box so they can move freely and not be affected by the box's
        // rotation.
        for y in (-20..=20).step_by(2) {
            for x in (-20..=20).step_by(2) {
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(1.0))),
                    MeshMaterial2d(materials.add(Color::srgb_u8(
                        fastrand::u8(0..=u8::MAX),
                        fastrand::u8(0..=u8::MAX),
                        fastrand::u8(0..=u8::MAX),
                    ))),
                    RigidBody::Dynamic,
                    Collider::ball(1.0),
                    Restitution::coefficient(0.99),
                    Transform::from_xyz(x as f32, y as f32, 0.0),
                    Velocity::linear(Vec2::new(10.0 * fastrand::f32() - 5.0, -fastrand::f32())),
                    Ccd::enabled(),
                    ColliderMassProperties::Density(0.5),
                ));
            }
        }
    }

    fn toggle_wireframe(
        mut wireframe_config: ResMut<Wireframe2dConfig>,
        keyboard: Res<ButtonInput<KeyCode>>,
    ) {
        if keyboard.just_pressed(KeyCode::Space) {
            wireframe_config.global = !wireframe_config.global;
        }
    }

    fn rotate_box(mut box_query: Query<&mut Transform, With<Box>>, time: Res<Time>) {
        for mut transform in box_query.iter_mut() {
            transform.rotation *= Quat::from_rotation_z(time.delta_secs() * 0.1);
        }
    }
}
