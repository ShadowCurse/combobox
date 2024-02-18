use bevy::prelude::*;
use rand::Rng;

use std::{f32::consts::PI, ops::Range};

use crate::physics::{Ball, Dynamic, Velocity};

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, spawn_controller);
        app.add_systems(Update, spawn_items);
        app.add_event::<SpawnItemEvent>();
        app.init_resource::<SpawnItemTimer>();
    }
}

pub const SPAWN_OFFSET: Vec3 = Vec3::new(0.0, 0.0, -1.0);
pub const SPAWN_RANGE: Range<u8> = 0..2;
pub const NUM_ITEMS: u8 = 5;
pub const ITEM_1_RADIUS: f32 = 5.0;
pub const ITEM_1_BOUNCINESS: f32 = 0.5;
pub const ITEM_1_COLOR: Color = Color::GRAY;
pub const ITEM_2_RADIUS: f32 = 8.0;
pub const ITEM_2_BOUNCINESS: f32 = 0.4;
pub const ITEM_2_COLOR: Color = Color::YELLOW_GREEN;
pub const ITEM_3_RADIUS: f32 = 13.0;
pub const ITEM_3_BOUNCINESS: f32 = 0.3;
pub const ITEM_3_COLOR: Color = Color::GREEN;
pub const ITEM_4_RADIUS: f32 = 16.0;
pub const ITEM_4_BOUNCINESS: f32 = 0.2;
pub const ITEM_4_COLOR: Color = Color::GOLD;
pub const ITEM_5_RADIUS: f32 = 19.0;
pub const ITEM_5_BOUNCINESS: f32 = 0.1;
pub const ITEM_5_COLOR: Color = Color::ORANGE_RED;

#[derive(Component)]
pub struct Platform {
    pub speed: f32,
    pub next_item: u8,
}

#[derive(Event)]
pub struct SpawnItemEvent {
    pub item_type: u8,
    pub position: Vec3,
}

#[derive(Resource)]
pub struct SpawnItemTimer {
    pub timer: Timer,
}

impl Default for SpawnItemTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

pub struct ItemResource {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub radius: f32,
    pub bounciness: f32,
}

#[derive(Resource)]
pub struct ItemsResources {
    pub resources: [ItemResource; NUM_ITEMS as usize],
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let item_1_mesh = meshes.add(
        Sphere {
            radius: ITEM_1_RADIUS,
        }
        .mesh()
        .build(),
    );
    let item_1_material = materials.add(ITEM_1_COLOR);
    let item_2_mesh = meshes.add(
        Sphere {
            radius: ITEM_2_RADIUS,
        }
        .mesh()
        .build(),
    );
    let item_2_material = materials.add(ITEM_2_COLOR);
    let item_3_mesh = meshes.add(
        Sphere {
            radius: ITEM_3_RADIUS,
            ..default()
        }
        .mesh()
        .build(),
    );
    let item_3_material = materials.add(ITEM_3_COLOR);
    let item_4_mesh = meshes.add(
        Sphere {
            radius: ITEM_4_RADIUS,
            ..default()
        }
        .mesh()
        .build(),
    );
    let item_4_material = materials.add(ITEM_4_COLOR);
    let item_5_mesh = meshes.add(
        Sphere {
            radius: ITEM_5_RADIUS,
            ..default()
        }
        .mesh()
        .build(),
    );
    let item_5_material = materials.add(ITEM_5_COLOR);

    commands.insert_resource(ItemsResources {
        resources: [
            ItemResource {
                mesh: item_1_mesh,
                material: item_1_material,
                radius: ITEM_1_RADIUS,
                bounciness: ITEM_1_BOUNCINESS,
            },
            ItemResource {
                mesh: item_2_mesh,
                material: item_2_material,
                radius: ITEM_2_RADIUS,
                bounciness: ITEM_2_BOUNCINESS,
            },
            ItemResource {
                mesh: item_3_mesh,
                material: item_3_material,
                radius: ITEM_3_RADIUS,
                bounciness: ITEM_3_BOUNCINESS,
            },
            ItemResource {
                mesh: item_4_mesh,
                material: item_4_material,
                radius: ITEM_4_RADIUS,
                bounciness: ITEM_4_BOUNCINESS,
            },
            ItemResource {
                mesh: item_5_mesh,
                material: item_5_material,
                radius: ITEM_5_RADIUS,
                bounciness: ITEM_5_BOUNCINESS,
            },
        ],
    });

    let mesh = meshes.add(
        Capsule3d {
            radius: 2.0,
            half_length: 5.0,
        }
        .mesh(),
    );
    let material = materials.add(Color::GOLD);

    let mut transform = Transform::from_rotation(Quat::from_rotation_z(PI / 2.0));
    transform.translation = Vec3::new(50.0, 0.0, 100.0);
    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform,
            ..default()
        })
        .insert(Platform {
            speed: 100.0,
            next_item: 0,
        });
}

fn spawn_controller(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut spawn_item_timer: ResMut<SpawnItemTimer>,
    mut spawn_item_events: EventWriter<SpawnItemEvent>,
    mut platform: Query<(&mut Transform, &mut Platform)>,
) {
    let (mut platform_transform, mut platform) = match platform.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut dir = None;
    if keys.pressed(KeyCode::KeyA) {
        dir = Some(Vec3::NEG_X);
    }
    if keys.pressed(KeyCode::KeyD) {
        dir = Some(Vec3::X);
    }

    spawn_item_timer.timer.tick(time.delta());
    if keys.pressed(KeyCode::Space) && spawn_item_timer.timer.finished() {
        spawn_item_events.send(SpawnItemEvent {
            item_type: platform.next_item,
            position: platform_transform.translation + SPAWN_OFFSET,
        });
        platform.next_item = rand::thread_rng().gen_range(SPAWN_RANGE);
    }

    if let Some(dir) = dir {
        platform_transform.translation += dir * time.delta().as_secs_f32() * platform.speed;
    }
}

fn spawn_items(
    items_resources: Res<ItemsResources>,
    mut spawn_item_events: EventReader<SpawnItemEvent>,
    mut commands: Commands,
) {
    for event in spawn_item_events.read() {
        let resources = &items_resources.resources[event.item_type as usize];

        commands
            .spawn(PbrBundle {
                mesh: resources.mesh.clone(),
                material: resources.material.clone(),
                transform: Transform::from_translation(event.position),
                ..default()
            })
            .insert(Ball {
                radius: resources.radius,
                bounciness: resources.bounciness,
                ball_type: event.item_type,
            })
            .insert(Dynamic)
            .insert(Velocity {
                velocity: Vec3::default(),
            });
    }
}
