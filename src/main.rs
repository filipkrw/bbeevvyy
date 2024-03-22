use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, print_mouse_events_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(20.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    commands
        .spawn(Collider::cuboid(10.0, 1.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.5, 0.0)));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: false,
            intensity: 5000000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(7.5, 15.0, 7.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn print_mouse_events_system(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

                commands.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_translation(hit_point),
                    ..default()
                });
            }
        }
    }
}
