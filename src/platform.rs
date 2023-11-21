use bevy::prelude::*;

use std::f32::consts::PI;

use crate::physics::{Ball, Dynamic, Velocity};

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_phatform);
        app.add_systems(Update, spawn_controller);
        app.add_systems(Update, spawn_items);
        app.add_event::<ItemDropEvent>();
        app.init_resource::<ItemDropTimer>();
    }
}

#[derive(Component)]
struct Platform {
    speed: f32,
}

#[derive(Event)]
struct ItemDropEvent;

#[derive(Resource)]
struct ItemDropTimer {
    timer: Timer,
}

impl Default for ItemDropTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

fn spawn_phatform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(
        shape::Capsule {
            radius: 2.0,
            depth: 5.0,
            ..default()
        }
        .into(),
    );
    let material = materials.add(Color::GOLD.into());

    let mut transform = Transform::from_rotation(Quat::from_rotation_z(PI / 2.0));
    transform.translation = Vec3::new(50.0, 0.0, 100.0);
    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform,
            ..default()
        })
        .insert(Platform { speed: 100.0 });
}

fn spawn_controller(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut spawn_item_timer: ResMut<ItemDropTimer>,
    mut spawn_item_events: EventWriter<ItemDropEvent>,
    mut platform: Query<(&mut Transform, &Platform)>,
) {
    let (mut platform_transform, platform) = match platform.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut dir = None;
    if keys.pressed(KeyCode::A) {
        dir = Some(Vec3::NEG_X);
    }
    if keys.pressed(KeyCode::D) {
        dir = Some(Vec3::X);
    }

    spawn_item_timer.timer.tick(time.delta());
    if keys.pressed(KeyCode::Space) && spawn_item_timer.timer.finished() {
        spawn_item_events.send(ItemDropEvent);
    }

    if let Some(dir) = dir {
        platform_transform.translation += dir * time.delta().as_secs_f32() * platform.speed;
    }
}

fn spawn_items(
    platform: Query<&Transform, With<Platform>>,
    mut spawn_item_events: EventReader<ItemDropEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let platform_transform = match platform.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    for _ in spawn_item_events.read() {
        let mesh = meshes.add(
            shape::UVSphere {
                radius: 10.0,
                ..default()
            }
            .into(),
        );
        let material = materials.add(Color::WHITE.into());
        let mut transform = *platform_transform;
        transform.translation.z -= 10.0;

        commands
            .spawn(PbrBundle {
                mesh,
                material,
                transform,
                ..default()
            })
            .insert(Ball { radius: 10.0 })
            .insert(Dynamic)
            .insert(Velocity {
                velocity: Vec3::default(),
            });
    }
}
