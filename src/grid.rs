use bevy::prelude::*;
use crate::resources_and_components::GridCoord;

use crate::settings::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub const SQUARE_SIDE_SIZE: f32 = 50.;

pub fn get_xy_coords_from_screen_space_position(pos: &Vec2) -> (u32, u32) {
    ((pos.x / SQUARE_SIDE_SIZE) as u32,
     (pos.y / SQUARE_SIDE_SIZE) as u32)
}

pub fn block_position_to_screen_space_position(pos: &Vec2) -> Vec2 {
    *pos + Vec2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.)
        - Vec2::new((SQUARE_SIDE_SIZE as f32) / 2., (SQUARE_SIDE_SIZE as f32) / 2.)
}

pub fn screen_space_position_to_block_position(pos: &Vec2) -> Vec2 {
   *pos - Vec2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.)
        + Vec2::new((SQUARE_SIDE_SIZE as f32) / 2., (SQUARE_SIDE_SIZE as f32) / 2.)
}

pub fn get_aligned_pos_from_coords(xy_cords: &GridCoord) -> Vec2 {
    let x_pos: f32 = xy_cords.x as f32 * SQUARE_SIDE_SIZE;
    let y_pos: f32 = xy_cords.y as f32 * SQUARE_SIDE_SIZE;

    screen_space_position_to_block_position(&(x_pos, y_pos).into())
}


pub fn square_sprite() -> Sprite {
    Sprite::new(Vec2::new(SQUARE_SIDE_SIZE, SQUARE_SIDE_SIZE))
}

pub fn is_coordinate_within_borders(coord: &GridCoord) -> bool {
    if coord.x < 0 || coord.y < 0 {
        return false;
    }
    let count_x = (WINDOW_WIDTH / SQUARE_SIDE_SIZE) as u32;
    let count_y = (WINDOW_HEIGHT / SQUARE_SIDE_SIZE) as u32;
    if coord.x < count_x && coord.y < count_y {
        return true;
    }
    false
}
