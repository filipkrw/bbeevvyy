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
                mesh: meshes.add(Sphere::new(0.25)),
                material: materials.add(Color::BLUE),
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(0.25))
            .insert(ColliderMassProperties::Density(1.0))
            .insert(Velocity::default())
            .insert(GravityScale(1.0))
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(ExternalForce::default());
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            target: Default::default(),
        }
    }
}
