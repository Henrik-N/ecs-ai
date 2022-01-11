use bevy::input::InputPlugin;
use bevy::log;
use bevy::prelude::*;
use std::borrow::Borrow;
use std::ops::Deref;
use std::thread::{current, spawn};

use crate::application::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::input::{MouseLeftEvent, MousePos, MouseRightEvent, PlayerInputPlugin};
use crate::maze::{Coord, Maze, Symbol, SymbolConsts};
use crate::{grid_plugin, Player};
pub use resources::*;

use crate::application::GameState;

struct SpawnWallData {
    spawn_pos: Vec2,
    sprite: Sprite,
    material: Handle<ColorMaterial>,
}

pub use entities::*;
mod entities {
    use crate::{movement, MazeResource};
    use bevy::prelude::*;

    pub struct Wall;

    impl Wall {
        /// Spawns a block with a static collider
        pub(crate) fn spawn(
            mut cmd: &mut Commands,
            maze: &MazeResource,
            translation: Vec2,
        ) -> Entity {
            cmd.spawn_bundle(SpriteBundle {
                //material: mats.get("white"),
                transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.)),
                sprite: maze.square_sprite(Color::WHITE),
                ..Default::default()
            })
            .insert(movement::Collider::Solid)
            .id()
        }
    }
}

mod resources {
    use crate::maze::{Coord, Maze, Symbol, SymbolConsts, Wall};
    use bevy::log;
    use bevy::prelude::*;
    use derive_more::{Deref, DerefMut};
    use std::collections::HashMap;

    use crate::{setup_entities, Enemy, Player};
    use std::default::Default;

    #[derive(Deref, DerefMut, Component)]
    pub struct MazeResource {
        #[deref]
        #[deref_mut]
        pub loaded_maze: Maze,
        pub square_block_side_length: f32,
        pub screen_dimensions: (f32, f32),
        // entity id spawned at each coordinate
        pub spawned_entities: HashMap<Coord, Entity>,
    }

    impl MazeResource {
        pub fn free_coord(&mut self, cmd: &mut Commands, coord: Coord) {
            if let Some(entity) = self.spawned_entities.remove(&coord) {
                cmd.entity(entity).despawn_recursive();
            }
            self.loaded_maze.grid.set(coord, Symbol::FREE);
        }

        pub fn spawn_entity(&mut self, cmd: &mut Commands, coord: Coord, symbol: Symbol) {
            log::info!("placed entity at: {:?}", coord);
            // set it on the loaded maze
            self.loaded_maze.grid.set(coord, symbol);
            // spawn
            let pos = self.screen_pos_from_maze_coord(coord);

            let entity = match symbol {
                Symbol::FREE => return,
                Symbol::BLOCKED => Wall::spawn(cmd, &self, pos),
                Symbol::PLAYER_SPAWN => Player::spawn(cmd, pos),
                Symbol::ENEMY_SPAWN => Enemy::spawn(cmd, pos),
                _ => {
                    panic!("not implemented {}", symbol)
                }
            };
            self.spawned_entities.insert(coord, entity);
        }

        pub fn square_sprite(&self, color: Color) -> Sprite {
            let side = self.square_block_side_length;

            Sprite {
                color,
                custom_size: Some(Vec2::new(side, side)),
                ..Default::default()
            }
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
                spawned_entities: HashMap::new(),
            }
        }

        pub fn maze_coord_from_screen_pos(&self, screen_pos: &Vec2) -> Coord {
            // invert y
            let maze_x = (screen_pos.x / self.square_block_side_length) as _;
            let maze_y = (screen_pos.y / self.square_block_side_length) as _;
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
}

const MAZE_SAVE_FILE: &str = "saves/save.txt";

pub struct MazePlugin;
impl MazePlugin {
    const DEPENDENCY: &'static str = "MazePlugin";
}

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        let maze_resource =
            MazeResource::create_from_screen_dimensions((WINDOW_WIDTH, WINDOW_HEIGHT), 50.);

        app.insert_resource(maze_resource).add_system_set(
            SystemSet::on_update(GameState::BuildMap)
                .after(PlayerInputPlugin::DEPENDENCY)
                .label(Self::DEPENDENCY)
                .with_system(Self::on_mouse_left.system())
                .with_system(Self::on_mouse_right.system())
                .with_system(Self::save_maze_play_game.system())
                .with_system(Self::load_maze.system()),
        );
    }
}

impl MazePlugin {
    /// add a block
    fn on_mouse_left(
        mut cmd: Commands,
        mut maze: ResMut<MazeResource>,
        mut mouse_left_events: EventReader<MouseLeftEvent>,
    ) {
        for mouse_left in mouse_left_events.iter() {
            // get coord in maze that we just clicked on
            let mouse_maze_coord = maze.maze_coord_from_screen_pos(&mouse_left.mouse_pos);

            let to_spawn = {
                if mouse_left.shift_held {
                    Symbol::PLAYER_SPAWN
                } else if mouse_left.ctrl_held {
                    Symbol::ENEMY_SPAWN
                } else {
                    Symbol::BLOCKED
                }
            };

            match to_spawn {
                Symbol::BLOCKED => {
                    maze.free_coord(&mut cmd, mouse_maze_coord);
                    maze.spawn_entity(&mut cmd, mouse_maze_coord, Symbol::BLOCKED);
                    log::info!("placed wall at: {:?}", mouse_maze_coord);
                }
                Symbol::ENEMY_SPAWN => {
                    maze.free_coord(&mut cmd, mouse_maze_coord);
                    maze.spawn_entity(&mut cmd, mouse_maze_coord, Symbol::ENEMY_SPAWN);
                    log::info!("placed wall at: {:?}", mouse_maze_coord);
                }
                Symbol::PLAYER_SPAWN => {
                    // ensure there isn't more than 1 player spawn
                    if let Some(current_coord) = maze.loaded_maze.player_spawn_coord() {
                        maze.free_coord(&mut cmd, current_coord);
                    }
                    log::info!("placed player at: {:?}", mouse_maze_coord);
                    maze.spawn_entity(&mut cmd, mouse_maze_coord, Symbol::PLAYER_SPAWN);
                }
                _ => panic!("can't spawn: {}", to_spawn),
            }
        }
    }

    // delete a block
    fn on_mouse_right(
        mut cmd: Commands,
        mut maze: ResMut<MazeResource>,
        mut mouse_right_events: EventReader<MouseRightEvent>,
    ) {
        for mouse_right in mouse_right_events.iter() {
            let maze_coord = maze.maze_coord_from_screen_pos(&mouse_right.mouse_pos);

            maze.free_coord(&mut cmd, maze_coord);
        }
    }

    fn save_maze_play_game(
        mut maze: ResMut<MazeResource>,
        input: Res<Input<KeyCode>>,
        mut state: ResMut<State<GameState>>,
    ) {
        let save_key = input.just_pressed(KeyCode::S) || input.just_pressed(KeyCode::K);
        let play_key = input.just_pressed(KeyCode::P);
        // save file
        if save_key || play_key {
            maze.save_to_file(MAZE_SAVE_FILE);
        }

        if play_key {
            state.set(GameState::PlayGame);
        }
    }

    fn load_maze(mut cmd: Commands, mut maze: ResMut<MazeResource>, input: Res<Input<KeyCode>>) {
        // load file
        if input.just_pressed(KeyCode::L) {
            for (_coord, entity) in maze.spawned_entities.drain() {
                cmd.entity(entity).despawn_recursive();
            }

            let new_maze = Maze::load_from_file(MAZE_SAVE_FILE);

            let mut player_set = false;
            for (coord, &symbol) in new_maze.grid.iter_rows_first_enumerated() {
                if symbol == Symbol::PLAYER_SPAWN {
                    if player_set {
                        let err_msg = "you may only have one player spawn!";
                        log::error!(err_msg);
                        panic!("{}", err_msg);
                    }
                    player_set = true;
                }
                maze.spawn_entity(&mut cmd, coord, symbol);
            }

            for maze_coord in new_maze.blocked_coords() {
                let screen_pos = maze.screen_pos_from_maze_coord(maze_coord);

                let wall_entity = Wall::spawn(&mut cmd, &maze, screen_pos);
                maze.spawned_entities.insert(maze_coord, wall_entity);
            }

            maze.loaded_maze = new_maze;
        }
    }
}
