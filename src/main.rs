mod behaviors;
mod enemy;
mod player;

use behaviors::{
    arrive::Arrive,
    behavior::SteeringBehavior,
    collision_avoidance::{CollisionAvoidance, Entity},
};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use enemy::Enemy;
use player::Player;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, setup_enemies).chain())
        .add_systems(Update, handle_player_input)
        .add_systems(FixedUpdate, (move_player, move_enemies).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floor
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(20.0, 0.5, 20.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::default(),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 0.25, 10.0))
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        });

    // Walls
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 6.0, 20.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(10.0, 0.25, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.25, 10.0, 10.0),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 6.0, 20.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-10.0, 0.25, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.25, 10.0, 10.0),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(20.0, 6.0, 0.5)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.25, 10.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 10.0, 0.25),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(20.0, 6.0, 0.5)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.25, -10.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(10.0, 10.0, 0.25),
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: false,
            intensity: 5000000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(7.5, 15.0, 7.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Player
    Player::spawn(commands, meshes, materials);
}

fn setup_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng(); // Get a random number generator
    let positions = (0..200)
        .map(|_| {
            Vec3::new(
                rng.gen_range(-10.0..10.0), // Random x within a range, adjust as needed
                0.25,                       // y is always 0.25
                rng.gen_range(-10.0..10.0), // Random z within a range, adjust as needed
            )
        })
        .collect::<Vec<_>>();
    for position in positions {
        Enemy::spawn(&mut commands, &mut meshes, &mut materials, position)
    }
}

fn move_enemies(
    time: Res<Time>,
    q_player: Query<(&Player, &Transform, &Velocity)>,
    mut q_enemies: Query<(&Enemy, &Transform, &Velocity, &mut ExternalForce)>,
) {
    let (_, p_transform, p_velocity) = q_player.single();

    let enemy_entities: Vec<_> = q_enemies
        .into_iter()
        .map(|(_, e_transform, e_velocity, _)| Entity {
            position: e_transform.translation,
            velocity: e_velocity.linvel,
        })
        .enumerate()
        .collect();

    for (outer_index, enemy) in q_enemies.iter_mut().enumerate() {
        let (_, transform, velocity, mut ext_force) = enemy;

        let mut targets: Vec<Entity> = enemy_entities
            .clone()
            .into_iter()
            .filter(|(inner_index, _)| *inner_index != outer_index)
            .map(|(_, enemy)| enemy)
            .collect();

        targets.push(Entity {
            position: p_transform.translation,
            velocity: p_velocity.linvel,
        });

        let steering = CollisionAvoidance {
            current: Entity {
                position: transform.translation,
                velocity: velocity.linvel,
            },
            max_acceleration: 3000.0,
            radius: 0.5,
            targets,
        };
        let force = steering.get_steering_force();

        ext_force.force = force * time.delta_seconds();
    }
}

fn move_player(
    time: Res<Time>,
    mut set: ParamSet<(
        Query<(&Player, &mut Transform, &Velocity, &mut ExternalForce)>,
        Query<(&Enemy, &Transform, &Velocity)>,
    )>,
) {
    let enemies: Vec<_> = set
        .p1()
        .into_iter()
        .map(|(_, e_transform, e_velocity)| Entity {
            position: e_transform.translation,
            velocity: e_velocity.linvel,
        })
        .collect();

    let mut q_player = set.p0();

    let (player, p_transform, p_velocity, mut p_ext_force) = q_player.single_mut();

    let collision_avoidance = CollisionAvoidance {
        current: Entity {
            position: p_transform.translation,
            velocity: p_velocity.linvel,
        },
        max_acceleration: 3000.0,
        radius: 0.5,
        targets: enemies,
    };

    let arrive = Arrive {
        current_pos: p_transform.translation,
        current_velocity: p_velocity.linvel,
        target_pos: player.target,
        ..default()
    };

    let arrive_force = arrive.get_steering_force();
    let collision_avoidance_force = collision_avoidance.get_steering_force();

    let direction = player.target - p_transform.translation;
    if direction.length() < 0.01 {
        p_ext_force.force = Vec3::ZERO;
        return;
    }

    let force = arrive_force * 0.3 + collision_avoidance_force * 0.7;
    p_ext_force.force = force * time.delta_seconds();
}

fn handle_player_input(
    mut query: Query<&mut Player>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier: Res<RapierContext>,
) {
    let window = windows.single();

    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            let (camera, camera_transform) = cameras.single();
            let Some(cursor_position) = window.cursor_position() else {
                return;
            };
            let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
                return;
            };
            let ray_direction = ray.direction.normalize();

            if let Some((entity, toi)) = rapier.cast_ray(
                ray.origin,
                ray_direction,
                100.0,
                true,
                QueryFilter::default(),
            ) {
                let hit_point = camera_transform.translation() + ray_direction * toi;
                println!("Hit entity {:?} at {:?}", entity, hit_point);

                let mut player = query.single_mut();
                player.target = hit_point;
            }
        }
    }
}
