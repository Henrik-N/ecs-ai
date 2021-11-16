// General resources and components


use bevy::prelude::*;


enum Collider {
    Solid,
}
impl Default for Collider {
    fn default() -> Self {
        Self::Solid
    }
}

// Tag components -------
pub struct PlayerTag;


// Components ----------------


#[derive(Bundle, Default)]
struct BlockedBlock {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

#[derive(Default)]
pub struct Movement {
    pub speed: f32,
}


// Resources ----------------

#[derive(Default)]
pub struct MousePos(pub Vec2);
impl MousePos {
    pub fn get(&self) -> Vec2 {
        self.0.clone()
    }
}


#[derive(Default)]
pub struct AxisInput {
    pub axis: Vec2,
}
