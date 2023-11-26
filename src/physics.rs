use bevy::{prelude::*, utils::petgraph::visit::IntoNodeReferences};

const GRAVITY: f32 = 100.0;
const MAX_SPEED: f32 = 100.0;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicsSystems {
    Movement,
    CollisionDetection,
    CollisionResolution,
}

pub struct PhysicsPlugin {
    pub debug: bool,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();

        app.configure_sets(
            Update,
            PhysicsSystems::Movement.before(PhysicsSystems::CollisionDetection),
        );
        app.configure_sets(
            Update,
            PhysicsSystems::CollisionDetection.before(PhysicsSystems::CollisionResolution),
        );

        app.add_systems(Update, balls_movement.in_set(PhysicsSystems::Movement));
        app.add_systems(
            Update,
            (ball_rect_collision_system, ball_ball_collision_system)
                .in_set(PhysicsSystems::CollisionDetection),
        );
        app.add_systems(
            Update,
            balls_collision_resolution.in_set(PhysicsSystems::CollisionResolution),
        );

        if self.debug {
            app.add_systems(
                Update,
                debug_physics_event.in_set(PhysicsSystems::CollisionResolution),
            );
            app.add_systems(Update, debug_physics_rect);
        }
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub velocity: Vec3,
}

#[derive(Component, Debug)]
pub struct Ball {
    pub radius: f32,
    pub bounciness: f32,
}

#[derive(Component, Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Debug)]
pub struct Dynamic;

#[derive(Debug, Event)]
pub struct CollisionEvent {
    pub entity1: Entity,
    pub entity2: Entity,
    pub collision_point: Vec2,
}

fn ball_rect_collision_system(
    mut collision_events: EventWriter<CollisionEvent>,
    balls: Query<(Entity, &Ball, &Transform), With<Dynamic>>,
    rectangles: Query<(Entity, &Rectangle, &Transform)>,
) {
    for (ball_entity, ball, ball_transform) in balls.iter() {
        for (rect_entity, rect, rect_transform) in rectangles.iter() {
            if let Some(collision_point) =
                ball_rect_collision(ball, ball_transform, rect, rect_transform)
            {
                collision_events.send(CollisionEvent {
                    entity1: ball_entity,
                    entity2: rect_entity,
                    collision_point,
                });
            }
        }
    }
}

fn ball_rect_collision(
    ball: &Ball,
    ball_transform: &Transform,
    rect: &Rectangle,
    rect_transform: &Transform,
) -> Option<Vec2> {
    let mut px = ball_transform.translation.x;
    let mut pz = ball_transform.translation.z;
    px = px.max(rect_transform.translation.x - rect.width / 2.0);
    px = px.min(rect_transform.translation.x + rect.width / 2.0);
    pz = pz.max(rect_transform.translation.z - rect.height / 2.0);
    pz = pz.min(rect_transform.translation.z + rect.height / 2.0);

    if (ball_transform.translation.x - px).powi(2) + (ball_transform.translation.z - pz).powi(2)
        < ball.radius.powi(2)
    {
        Some(Vec2::new(px, pz))
    } else {
        None
    }
}

fn ball_ball_collision_system(
    mut collision_events: EventWriter<CollisionEvent>,
    balls: Query<(Entity, &Ball, &Transform), With<Dynamic>>,
) {
    for [(ball_1_entity, ball_1, ball_1_transform), (ball_2_entity, ball_2, ball_2_transform)] in
        balls.iter_combinations()
    {
        if let Some(collision_point) =
            ball_ball_collision(ball_1, ball_1_transform, ball_2, ball_2_transform)
        {
            collision_events.send(CollisionEvent {
                entity1: ball_1_entity,
                entity2: ball_2_entity,
                collision_point,
            });
            collision_events.send(CollisionEvent {
                entity1: ball_2_entity,
                entity2: ball_1_entity,
                collision_point,
            });
        }
    }
}

fn ball_ball_collision(
    ball_1: &Ball,
    ball_1_transform: &Transform,
    ball_2: &Ball,
    ball_2_transform: &Transform,
) -> Option<Vec2> {
    let v = ball_1_transform.translation - ball_2_transform.translation;
    let radius_sum = ball_1.radius + ball_2.radius;
    let length = v.length();
    if length < radius_sum {
        let delta = (radius_sum - length) / 4.0;
        let offset = v.normalize() * (ball_2.radius - delta);
        let center = ball_2_transform.translation + offset;
        Some(Vec2::new(center.x, center.z))
    } else {
        None
    }
}

fn debug_physics_event(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in collision_events.read() {
        debug!("collision event: {:?}", event);
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(event.collision_point.x, 2.0, event.collision_point.y),
            ..default()
        });
    }
}

fn debug_physics_rect(
    rects: Query<(&Transform, &Rectangle)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut run: Local<bool>,
) {
    if *run {
        return;
    }
    for (transform, rectangle) in rects.iter() {
        println!("lol");
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                rectangle.width,
                10.0,
                rectangle.height,
            ))),
            material: materials.add(Color::YELLOW_GREEN.into()),
            transform: Transform::from_translation(transform.translation),
            ..default()
        });
    }
    *run = true;
}

fn balls_movement(time: Res<Time>, mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    for (mut transform, mut velocity) in balls.iter_mut() {
        velocity.velocity.z =
            (velocity.velocity.z - GRAVITY * time.delta().as_secs_f32()).max(-MAX_SPEED);
        transform.translation += velocity.velocity * time.delta().as_secs_f32();
    }
}

fn balls_collision_resolution(
    mut collision_events: EventReader<CollisionEvent>,
    mut balls: Query<(Entity, &Ball, &mut Velocity, &mut Transform), With<Dynamic>>,
) {
    for event in collision_events.read() {
        for (ball_entity, ball, mut ball_velocity, mut ball_transform) in balls.iter_mut() {
            if ball_entity == event.entity1 {
                let normal = (ball_transform.translation.xz() - event.collision_point).normalize();
                let velocity = ball_velocity.velocity.xz();

                let reflected = velocity - 2.0 * (velocity.dot(normal)) * normal;
                let reflected = Vec3::new(reflected.x, 0.0, reflected.y);
                ball_velocity.velocity = reflected * ball.bounciness;

                let collision_point =
                    Vec3::new(event.collision_point.x, 0.0, event.collision_point.y);
                let normal = Vec3::new(normal.x, 0.0, normal.y).normalize();
                ball_transform.translation = collision_point + normal * ball.radius;
            }
        }
    }
}
