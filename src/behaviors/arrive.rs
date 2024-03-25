use super::behavior::SteeringBehavior;
use bevy::math::Vec3;

pub struct Arrive {
    pub current_pos: Vec3,
    pub current_velocity: Vec3,
    pub target_pos: Vec3,
    pub slow_radius: f32,
    pub max_speed: f32,
    pub time_to_target: f32,
    pub max_acceleration: f32,
    pub max_deceleration: f32,
}

impl SteeringBehavior for Arrive {
    fn get_steering_force(&self) -> Vec3 {
        let direction = self.target_pos - self.current_pos;
        let distance = direction.length();

        let target_speed: f32 = if distance > self.slow_radius {
            self.max_speed
        } else {
            self.max_speed * distance / self.slow_radius
        };

        let force = direction.normalize() * target_speed;
        let force = force - self.current_velocity;
        let force = force / self.time_to_target;

        if force.dot(direction) > 0.0 {
            return force.clamp_length_max(self.max_acceleration);
        } else {
            return force.clamp_length_max(self.max_deceleration);
        }
    }
}

impl Default for Arrive {
    fn default() -> Self {
        Self {
            current_pos: Vec3::ZERO,
            current_velocity: Vec3::ZERO,
            target_pos: Vec3::ZERO,
            slow_radius: 1.5,
            max_speed: 7.5,
            time_to_target: 0.001,
            max_acceleration: 250.0,
            max_deceleration: 500.0,
        }
    }
}
