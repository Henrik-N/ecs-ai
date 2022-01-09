use crate::input::{MousePos, PlayerInputPlugin};
use crate::{grid_plugin, Mats};
use bevy::prelude::*;

pub use resources::*;
mod resources {
    use crate::{GridCoord, Mats};
    use bevy::prelude::*;
    use std::ops::Sub;

    // the coordinates players and enemies can't walk on
    #[derive(Default)]
    pub struct BlockedCoords(pub Vec<GridCoord>);
}

pub use components::*;
mod components {
    use std::ops::Sub;

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
    impl From<(u32, u32)> for GridCoord {
        fn from((x, y): (u32, u32)) -> Self {
            Self { x, y }
        }
    }
    impl GridCoord {
        pub fn new(x: u32, y: u32) -> Self {
            Self { x, y }
        }
    }
}

pub use entities::*;
mod entities {
    use crate::{grid_plugin, Commands, Mats, Res, SpriteBundle, Transform};

    #[derive(Default)]
    /// block shown when hovering around with the mouse over the grid
    pub struct PreviewWallBlock {
        pub enabled: bool,
    }
    impl PreviewWallBlock {
        pub(super) fn spawn(cmd: &mut Commands, mats: &Res<Mats>) {
            fn wall_sprite_bundle(cmd: &mut Commands, mats: &Res<Mats>) -> SpriteBundle {
                let wall_mat = mats.get("white");
                let square = grid_plugin::square_sprite();

                SpriteBundle {
                    material: wall_mat,
                    transform: Transform::from_xyz(0., 0., 0.),
                    sprite: square,
                    ..Default::default()
                }
            }
        }
    }
}

pub struct GridPlugin;
impl GridPlugin {
    pub const DEPENDENCY: &'static str = "grid_plugin";
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::plugin_startup.system())
            .insert_resource(BlockedCoords::default())
            .add_system_set(
                SystemSet::new()
                    .label(Self::DEPENDENCY)
                    .after(PlayerInputPlugin::DEPENDENCY)
                    .with_system(Self::preview_wall_block_follow_mouse.system()),
            );
    }
}

impl GridPlugin {
    fn plugin_startup(mut cmd: Commands, mats: Res<Mats>) {
        PreviewWallBlock::spawn(&mut cmd, &mats);
    }

    fn preview_wall_block_follow_mouse(
        mouse_pos: Res<MousePos>,
        mut q: Query<(&mut Transform, &Sprite, &PreviewWallBlock)>,
    ) {
        if let Ok((mut transform, sprite, wall_block)) = q.single_mut() {
            if !wall_block.enabled {
                return;
            }

            let xy_coords =
                grid_plugin::get_xy_coords_from_screen_space_position(&mouse_pos).into();
            let aligned_cords = grid_plugin::get_aligned_pos_from_coords(&xy_coords);

            let translation = Vec3::new(aligned_cords.x, aligned_cords.y, 0.);

            transform.translation = translation;
        }
    }
}

use bevy::prelude::*;

use crate::application::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const SQUARE_SIDE_SIZE: f32 = 50.;

pub fn get_xy_coords_from_screen_space_position(pos: &Vec2) -> (u32, u32) {
    (
        (pos.x / SQUARE_SIDE_SIZE) as u32,
        (pos.y / SQUARE_SIDE_SIZE) as u32,
    )
}

pub fn block_position_to_screen_space_position(pos: &Vec2) -> Vec2 {
    *pos + Vec2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.)
        - Vec2::new(
            (SQUARE_SIDE_SIZE as f32) / 2.,
            (SQUARE_SIDE_SIZE as f32) / 2.,
        )
}

pub fn screen_space_position_to_block_position(pos: &Vec2) -> Vec2 {
    *pos - Vec2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.)
        + Vec2::new(
            (SQUARE_SIDE_SIZE as f32) / 2.,
            (SQUARE_SIDE_SIZE as f32) / 2.,
        )
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
