use crate::util::file_io;
use crate::util::Array2D;
use anyhow::*;
use bevy::log;
use bevy::prelude::Vec2;
use derive_more::{Deref, DerefMut};
use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{Filter, FilterMap, Flatten, Map};
use std::ops::Range;
use std::slice::Iter;

pub type Coord = (usize, usize);

pub type Symbol = char;
pub trait SymbolConsts {
    const PLAYER_SPAWN: char = 'P';
    const ENEMY_SPAWN: char = 'E';
    const FREE: char = '.';
    // nothing on the tile
    const BLOCKED: char = '#';
    const END_OF_LINE: char = '\n';
}
impl SymbolConsts for Symbol {}

#[derive(Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Maze {
    #[deref]
    #[deref_mut]
    pub grid: Array2D<Symbol>,
}

impl Maze {
    pub fn player_spawn_coord(&self) -> Option<Coord> {
        self.grid
            .iter_rows_first_enumerated()
            .find_map(|(coord, &symbol)| {
                if symbol == Symbol::PLAYER_SPAWN {
                    Some(coord)
                } else {
                    None
                }
            })
    }

    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            grid: Array2D::new(width, height, Symbol::FREE),
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let str = self.grid.to_string();
        file_io::write_to_path(path, str.as_bytes()).expect("failed to save maze");
    }

    pub fn load_from_file(path: &str) -> Self {
        let maze: String = file_io::read_file_to_string(path).expect("failed to load maze file");
        let grid = Array2D::<Symbol>::from(maze);
        Self { grid }
    }

    pub fn blocked_coords(&self) -> Vec<Coord> {
        let height_range = (0..self.grid.height);
        let width_range = (0..self.grid.width);

        height_range
            .map(|y| {
                width_range.clone().filter_map(move |x| {
                    let coord = (x, y);
                    if *self.grid.get(coord) == Symbol::BLOCKED {
                        Some(coord)
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Maze: dimensions({}, {}),\nmaze: \n{}",
            self.grid.width,
            self.grid.height,
            self.grid.to_string()
        )
    }
}

#[test]
fn test_save_load_maze() {
    let mut maze = Maze::new_empty(10, 5);
    for x in 1..4 {
        maze.set((x, 3), Symbol::BLOCKED);
    }

    for y in 0..3 {
        maze.set((0, y), Symbol::ENEMY_SPAWN);
    }
    maze.set((9, 4), Symbol::PLAYER_SPAWN);

    // save and load
    let file_path = "saves/test_save.txt";
    maze.save_to_file(file_path);
    let loaded_maze = Maze::load_from_file(file_path);

    // assert equal
    assert_eq!(maze, loaded_maze);
}
