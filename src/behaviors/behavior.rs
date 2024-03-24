use bevy::math::Vec3;

pub trait SteeringBehavior {
    fn get_steering_force(&self) -> Vec3;
}
