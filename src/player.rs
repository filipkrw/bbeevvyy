use bevy::{math::Vec3, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player {
    pub target: Vec3,
}

impl Player {
    pub fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let position = Vec3 {
            y: 0.75,
            ..default()
        };
        commands
            .spawn(Player { target: position })
            .insert(PbrBundle {
                mesh: meshes.add(Sphere::new(0.5)),
                material: materials.add(Color::BLUE),
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(0.5))
            .insert(ColliderMassProperties::Density(3.0))
            .insert(Damping {
                // linear_damping: 0.8,
                ..default() // angular_damping: 0.8,
            })
            .insert(Velocity::default())
            .insert(GravityScale(1.0))
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(ExternalForce {
                // force: Vec3::new(0.0, 0.0, 0.0),
                ..default() // torque: Vec3::new(1.0, 2.0, 3.0),
            });
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            target: Default::default(),
        }
    }
}
