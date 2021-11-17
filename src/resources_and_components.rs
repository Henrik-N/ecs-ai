// General resources and components
use bevy::prelude::*;


// Tag components -------

pub struct Player {
    pub movement_speed: f32,
}


// Components ----------------

#[derive(Debug, Eq, PartialEq)]
pub enum SpriteCollider {
    Static,
    Dynamic
}

#[derive(Debug, Default, Clone)]
pub struct Velocity(pub Vec2);
impl From<Vec3> for Velocity {
    fn from(v: Vec3) -> Self {
        Self(Vec2::new(v.x, v.y))
    }
}
impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}


#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct GridCoord {
    pub x: u32,
    pub y: u32,
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
