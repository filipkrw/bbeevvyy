use std::f32::INFINITY;

use bevy::{log::info, math::Vec3, transform::components::Transform};
use bevy_rapier3d::dynamics::Velocity;

use super::behavior::SteeringBehavior;

pub struct CollisionAvoidance<'a> {
    pub max_acceleration: f32,
    pub radius: f32,
    pub current: (&'a Transform, &'a Velocity),
    pub targets: Vec<(&'a Transform, &'a Velocity)>,
}

impl SteeringBehavior for CollisionAvoidance<'_> {
    fn get_steering_force(&self) -> Vec3 {
        let time_horizon: f32 = 0.3;

        let mut total_force = Vec3::ZERO;

        for (target_transform, target_velocity) in self.targets.iter() {
            let time_to_collision =
                self.get_time_to_collision(&(*target_transform, *target_velocity));
            info!("{}", time_to_collision);

            if time_to_collision > time_horizon || time_to_collision == 0.0 {
                continue;
            }

            // let (target_transform, target_velocity) = target;
            let target_position = target_transform.translation;

            let (current_transform, current_velocity) = self.current;
            let current_position = current_transform.translation;

            let mut force = current_position + current_velocity.linvel * time_to_collision
                - target_position
                - target_velocity.linvel * time_to_collision;

            if force.x != 0.0 && force.y != 0.0 && force.z != 0.0 {
                force = force / f32::sqrt(force.dot(force));
            }

            let mut magnitude: f32 = 0.0;

            if time_to_collision >= 0.0 && time_to_collision <= time_horizon {
                magnitude =
                    self.max_acceleration * (time_horizon - time_to_collision) / time_horizon;
            }

            force *= magnitude;
            total_force += force;
        }

        return total_force;
    }
}

impl CollisionAvoidance<'_> {
    pub fn get_time_to_collision(&self, target: &(&Transform, &Velocity)) -> f32 {
        let r: f32 = 50.0 + 50.0;
        let (target_transform, target_velocity) = target;
        let target_position = target_transform.translation;

        let (current_transform, current_velocity) = self.current;
        let current_position = current_transform.translation;

        let x = target_position - current_position;
        let c = x.dot(x) - r * r;

        if c < 0.0 {
            return 0.0;
        }

        let v = current_velocity.linvel - target_velocity.linvel;
        let a = v.dot(v);
        let b = x.dot(x);
        let discr = b * b - a * c;

        if discr <= 0.0 {
            return INFINITY;
        }

        let tau = (b - f32::sqrt(discr)) / a;

        if tau < 0.0 {
            return INFINITY;
        }

        return tau;
    }
}
