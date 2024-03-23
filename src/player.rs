use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component)]
pub struct Player {
    pub target: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            target: Default::default(),
        }
    }
}
