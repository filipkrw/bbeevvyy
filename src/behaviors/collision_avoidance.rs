use std::f32::INFINITY;

use bevy::math::Vec3;

use super::behavior::SteeringBehavior;

pub struct Entity {
    pub position: Vec3,
    pub velocity: Vec3,
}

pub struct CollisionAvoidance {
    pub max_acceleration: f32,
    pub radius: f32,
    pub current: Entity,
    pub targets: Vec<Entity>,
}

impl SteeringBehavior for CollisionAvoidance {
    fn get_steering_force(&self) -> Vec3 {
        let time_horizon: f32 = 0.1;

        let mut total_force = Vec3::ZERO;

        for target in self.targets.iter() {
            let time_to_collision = self.get_time_to_collision(target);

            if time_to_collision > time_horizon || time_to_collision == 0.0 {
                continue;
            }

            let mut force = self.current.position + self.current.velocity * time_to_collision
                - target.position
                - target.velocity * time_to_collision;

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

impl CollisionAvoidance {
    pub fn get_time_to_collision(&self, target: &Entity) -> f32 {
        let r: f32 = 1.0 + 1.0;

        let x = target.position - self.current.position;
        let c = x.dot(x) - r * r;

        if c < 0.0 {
            return 0.0;
        }

        let v = self.current.velocity - target.velocity;
        let a = v.dot(v);
        let b = x.dot(x);
        let discr = b * b - a * c;

        if a == 0.0 {
            return INFINITY;
        }

        if discr > 0.0 {
            let tau = (b - f32::sqrt(discr)) / a;

            if tau < 0.0 {
                return INFINITY;
            }

            return tau;
        }

        return INFINITY;
    }
}
