use bevy::input::InputPlugin;
use bevy::log;
use bevy::prelude::*;
use std::ops::Deref;
use std::thread::spawn;

use crate::application::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::input::{MouseLeftEvent, MousePos, MouseRightEvent, PlayerInputPlugin};
use crate::maze::{Coord, Maze, Symbol, SymbolConsts};
use crate::{grid_plugin, Mats};
pub use resources::*;

use crate::application::GameState;

mod resources {
    use crate::maze::{Coord, Maze, Symbol, SymbolConsts};
    use bevy::log;
    use bevy::prelude::{Entity, Sprite, Vec2};
    use std::collections::HashMap;

    use derive_more::{Deref, DerefMut};

    #[derive(Deref, DerefMut)]
    pub struct MazeResource {
        #[deref]
        #[deref_mut]
        pub loaded_maze: Maze,
        pub square_block_side_length: f32,
        pub screen_dimensions: (f32, f32),
    }

    impl MazeResource {
        pub fn square_sprite(&self) -> Sprite {
            let side = self.square_block_side_length;
            Sprite::new(Vec2::new(side, side))
        }


        pub fn create_from_screen_dimensions(
            (screen_width, screen_height): (f32, f32),
            square_block_side_length: f32,
        ) -> Self {
            let grid_width = (screen_width.clone() / square_block_side_length.clone()) as _;
            let grid_height = (screen_height.clone() / square_block_side_length.clone()) as _;

            log::info!("grid width: {}, grid height: {}", grid_width, grid_height);
            let mut maze = Maze::new_empty(grid_width, grid_height);
            log::info!("{}", maze);

            Self {
                loaded_maze: maze,
                square_block_side_length,
                screen_dimensions: (screen_width, screen_height),
            }
        }

        pub fn maze_coord_from_screen_pos(&self, screen_pos: &Vec2) -> Coord {
            // invert y
            let maze_x = (screen_pos.x / &self.square_block_side_length) as _;
            let maze_y = (screen_pos.y / &self.square_block_side_length) as _;
            (maze_x, maze_y)
        }

        pub fn screen_pos_from_maze_coord(&self, (maze_x, maze_y): Coord) -> Vec2 {
            let square_side = self.square_block_side_length.clone();

            let x: f32 = maze_x as f32 * square_side.clone();
            let y: f32 = maze_y as f32 * square_side.clone();

            // bevy has origin at the center of the screen, I want it in the bottom left for this
            let half_square_side = square_side / 2.;

            let (window_width, window_height) = self.screen_dimensions.clone();
            let x = x - window_width / 2. + half_square_side.clone();
            let y = y - window_height / 2. + half_square_side;

            Vec2::new(x, y)
        }
    }

    #[derive(Default, Deref, DerefMut)]
    /// Keeps track of which blocks are spawned
    pub struct SpawnedMazeEntites(
        #[deref]
        #[deref_mut]
        HashMap<Coord, Entity>,
    );
}

const MAZE_SAVE_FILE: &str = "saves/save.txt";

pub struct MazePlugin;
impl MazePlugin {
    const DEPENDENCY: &'static str = "MazePlugin";
}

impl Plugin for MazePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let maze_resource =
            MazeResource::create_from_screen_dimensions((WINDOW_WIDTH, WINDOW_HEIGHT), 50.);

        let spawned_entities = SpawnedMazeEntites::default();

        app
            .insert_resource(maze_resource)
            .insert_resource(spawned_entities)
            .add_system_set(
                SystemSet::on_update(GameState::BuildMap)
                    .after(PlayerInputPlugin::DEPENDENCY)
                    .label(Self::DEPENDENCY)
                    .with_system(Self::on_mouse_left.system())
                    .with_system(Self::on_mouse_right.system())
                    .with_system(Self::save_load_maze.system()),
            );
    }
}

impl MazePlugin {
    /// add a block
    fn on_mouse_left(
        mut cmd: Commands,
        mats: Res<Mats>,
        mut maze: ResMut<MazeResource>,
        mut mouse_left_events: EventReader<MouseLeftEvent>,
        mut spawned_entities: ResMut<SpawnedMazeEntites>,
    ) {
        for mouse_left in mouse_left_events.iter() {
            // get coord in maze that we just clicked on
            let maze_coord = maze.maze_coord_from_screen_pos(&mouse_left.mouse_pos);

            if maze.grid.get(maze_coord) == &Symbol::FREE {
                log::info!("placed coord at: {:?}", maze_coord);

                maze.grid.set(maze_coord, Symbol::BLOCKED);

                let pos = maze.screen_pos_from_maze_coord(maze_coord);

                let wall_entity = spawn_wall(&mut cmd, &mats, pos);

                spawned_entities.insert(maze_coord, wall_entity);
            }
        }
    }

    // delete a block
    fn on_mouse_right(
        mut cmd: Commands,
        mut maze: ResMut<MazeResource>,
        mut mouse_right_events: EventReader<MouseRightEvent>,
        mut spawned_entities: ResMut<SpawnedMazeEntites>,
    ) {
        for mouse_right in mouse_right_events.iter() {
            let maze_coord = maze.maze_coord_from_screen_pos(&mouse_right.mouse_pos);

            if maze.grid.get(maze_coord) == &Symbol::BLOCKED {
                maze.grid.set(maze_coord, Symbol::FREE);

                if let Some(&entity) = spawned_entities.get(&maze_coord) {
                    cmd.entity(entity).despawn_recursive();
                    spawned_entities.remove(&maze_coord);
                } else {
                    panic!("no entry for {:?}", maze_coord);
                }
            }
        }
    }

    fn save_load_maze(
        mut cmd: Commands,
        mats: Res<Mats>,
        mut maze: ResMut<MazeResource>,
        input: Res<Input<KeyCode>>,
        mut spawned_entities: ResMut<SpawnedMazeEntites>,
    ) {
        // save file
        if input.just_pressed(KeyCode::K) {
            maze.save_to_file(MAZE_SAVE_FILE);
        }

        // load file
        if input.just_pressed(KeyCode::L) {
            for (_coord, entity) in spawned_entities.drain() {
                cmd.entity(entity).despawn_recursive();
            }

            let new_maze = Maze::load_from_file(MAZE_SAVE_FILE);

            for maze_coord in new_maze.blocked_coords() {
                //get_blocked_coords().into_iter() {
                let screen_pos = maze.screen_pos_from_maze_coord(maze_coord);

                let wall_entity = spawn_wall(&mut cmd, &mats, screen_pos);
                spawned_entities.insert(maze_coord, wall_entity);
            }

            maze.loaded_maze = new_maze;
        }
    }
}

struct SpawnWallData {
    spawn_pos: Vec2,
    sprite: Sprite,
    material: Handle<ColorMaterial>,
}

/// Spawns a block with a static collider
fn spawn_wall(mut cmd: &mut Commands, mats: &Res<Mats>, translation: Vec2) -> Entity {
    cmd.spawn_bundle(SpriteBundle {
        material: mats.get("white"),
        transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.)),
        sprite: grid_plugin::square_sprite(),
        ..Default::default()
    })
    //.insert(SpriteCollider::Static)
    //.insert(data.spawn_coords)
    .id()
}
