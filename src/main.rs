use bevy::prelude::*;

mod physics;
mod platform;
mod scene;

use physics::PhysicsPlugin;
use platform::PlatformPlugin;
use scene::ScenePlugin;

fn main() {
    let mut app = App::new();

    // app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });

    app.add_plugins(DefaultPlugins);
    app.add_plugins(PhysicsPlugin { debug: false });
    app.add_plugins(PlatformPlugin);
    app.add_plugins(ScenePlugin);

    app.add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 2.0, 2.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(150.0, -200.0, 100.0)
            .looking_at(Vec3::new(50.0, 0.0, 40.0), Vec3::Z),
        ..default()
    });

    // X axis
    let x_mesh = meshes.add(shape::Box::new(10.0, 1.0, 1.0).into());
    let x_material = materials.add(Color::RED.into());
    let x_transform = Transform::from_translation(Vec3::new(15.0, 10.0, 20.0));
    commands.spawn(PbrBundle {
        mesh: x_mesh,
        material: x_material,
        transform: x_transform,
        ..default()
    });

    // Y axis
    let y_mesh = meshes.add(shape::Box::new(1.0, 10.0, 1.0).into());
    let y_material = materials.add(Color::GREEN.into());
    let y_transform = Transform::from_translation(Vec3::new(10.0, 15.0, 20.0));
    commands.spawn(PbrBundle {
        mesh: y_mesh,
        material: y_material,
        transform: y_transform,
        ..default()
    });

    // Z axis
    let z_mesh = meshes.add(shape::Box::new(1.0, 1.0, 10.0).into());
    let z_material = materials.add(Color::BLUE.into());
    let z_transform = Transform::from_translation(Vec3::new(10.0, 10.0, 25.0));
    commands.spawn(PbrBundle {
        mesh: z_mesh,
        material: z_material,
        transform: z_transform,
        ..default()
    });
}
