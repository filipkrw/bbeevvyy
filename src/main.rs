mod behaviors;
mod player;

use behaviors::{arrive::Arrive, behavior::SteeringBehavior};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use player::Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_player_input, move_player).chain())
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

fn move_player(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform, &Velocity, &mut ExternalForce)>,
) {
    let (player, transform, velocity, mut ext_force) = query.single_mut();
    let direction = player.target - transform.translation;

    if direction.length() < 0.01 {
        ext_force.force = Vec3::ZERO;
        return;
    }

    let steering = Arrive {
        current_pos: transform.translation,
        current_velocity: velocity.linvel,
        target_pos: player.target,
        ..default()
    };

    let steering = steering.get_steering_force();

    ext_force.force = steering * time.delta_seconds();
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
