use bevy::{math::Vec3, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Enemy;

impl Enemy {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec3,
    ) {
        commands
            .spawn(Enemy)
            .insert(PbrBundle {
                mesh: meshes.add(Sphere::new(0.25)),
                material: materials.add(Color::RED),
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
