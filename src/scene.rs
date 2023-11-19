use bevy::prelude::*;

use std::f32::consts::PI;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene);
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Box::new(100.0, 1.0, 1.0).into());
    let material = materials.add(Color::GRAY.into());

    // Left wall
    let mut transform = Transform::from_rotation(Quat::from_rotation_y(-PI / 2.0));
    transform.translation = Vec3::new(0.0, 0.0, 50.0);
    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform,
        ..default()
    });

    // Right wall
    let mut transform = Transform::from_rotation(Quat::from_rotation_y(-PI / 2.0));
    transform.translation = Vec3::new(100.0, 0.0, 50.0);
    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform,
        ..default()
    });

    // Bottom wall
    let transform = Transform::from_translation(Vec3::new(50.0, 0.0, 0.0));
    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform,
        ..default()
    });
}
