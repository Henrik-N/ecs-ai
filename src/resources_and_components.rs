use std::ops::{Mul, Neg, Sub};
// General resources and components
use bevy::prelude::*;

// Tag components -------

pub struct Player {
    pub movement_speed: f32,
}


// Components ----------------
pub mod collisions {
    use std::ops::Deref;
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

pub use collisions::*;


//pub struct AccumulatedVelocity {
//    // the velocity the previous fixed update frame
//    pub previous_velocity: Vec2,
//    // the combined velocity since the last regular frame update
//    pub accumulated_velocity: Vec2,
//}


#[derive(Debug, Default, Clone)]
pub struct Velocity{
    pub velocity: Vec2, // accumulated velocity since the last frame update (resets on update after moving the objects)
    pub previous_velocity: Vec2,
}



//impl From<Vec3> for Velocity {
//    fn from(v: Vec3) -> Self {
//        Self(Vec2::new(v.x, v.y))
//    }
//}
//impl From<Vec2> for Velocity {
//    fn from(v: Vec2) -> Self {
//        Self(v)
//    }
//}


#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct GridCoord {
    pub x: u32,
    pub y: u32,
}
impl Sub for GridCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl GridCoord {
    pub fn new(x: u32, y: u32) -> Self {
        Self {x, y}
    }
}
impl From<(u32, u32)> for GridCoord {
    fn from((x, y): (u32, u32)) -> Self {
        Self{x, y}
    }
}

// Resources ----------------

#[derive(Default)]
pub struct BlockedCoords(pub Vec<GridCoord>);


#[derive(Debug, Default)]
pub struct MousePos(pub Vec2);
impl MousePos {
    pub fn get(&self) -> Vec2 {
        self.0.clone()
    }
}

#[derive(Debug, Default)]
pub struct AxisInput {
    pub axis: Vec2,
}
