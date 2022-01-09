pub mod array2d;
pub mod pathfinding;

pub mod file_io;

pub use array2d::Array2D;

use bevy::math::{Vec2, Vec3};

pub fn to_vec3(v: &Vec2) -> Vec3 {
    Vec3::new(v.x, v.y, 0.)
}

pub fn to_vec2(v: &Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}
