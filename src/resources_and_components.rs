use std::ops::{Mul, Neg, Sub};
// General resources and components
use bevy::prelude::*;

// Tag components -------

// Components ----------------
pub use collisions::*;
pub mod collisions {
    use bevy::prelude::*;
    use bevy::sprite::collide_aabb;
    use bevy::sprite::collide_aabb::Collision;

    #[derive(Debug)]
    pub enum CollidedWith {
        Static,
        Dynamic(Entity), // usize == index
    }

    #[derive(Debug)]
    pub struct CollisionData {
        pub collision_side: Collision,
        pub collided_with: CollidedWith,
        pub offset: Vec2, // vec to entity collided with
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum SpriteCollider {
        Static,
        Dynamic, // contains events if collided with something
    }
}

#[derive(Debug, Default, Clone)]
pub struct Velocity {
    pub velocity: Vec2, // accumulated velocity since the last frame update (resets on update after moving the objects)
    pub previous_velocity: Vec2,
}
