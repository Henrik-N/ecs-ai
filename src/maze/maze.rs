use derive_more::{Deref, DerefMut};
use std::fmt::{Debug, Display, Formatter};

use crate::util::file_io;

use anyhow::*;
use bevy::core::AsBytes;
use bevy::log;
use bevy::prelude::Vec2;

use bevy::render::pipeline::UniformProperty::Array;
use std::clone::Clone;
use std::iter::{Filter, FilterMap, Flatten, Map};
use std::ops::Range;
use std::slice::Iter;

use crate::util::Array2D;

pub type Coord = (usize, usize);

pub type Symbol = char;
pub trait SymbolConsts {
    const PLAYER_SPAWN: char = 'P';
    const ENEMY_SPAWN: char = 'E';
    const FREE: char = '.'; // nothing on the tile
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

mod old {
    use super::*;

    pub struct MazeOld {
        width: usize,
        height: usize,
        //
        grid: Array2D<Symbol>,
    }

    impl Display for MazeOld {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Maze: dimensions({}, {})\n---\n{}\n---\n",
                self.width,
                self.height,
                self.grid.to_string()
            )
        }
    }

    // creation
    impl MazeOld {
        pub fn new_empty(width: usize, height: usize) -> Self {
            Self {
                width,
                height,
                grid: Array2D::new(width, height, Symbol::FREE),
            }
        }
    }

    // save/load
    impl MazeOld {
        fn find_width(maze: &str) -> usize {
            let (first_line, _) = maze
                .split_once(Symbol::END_OF_LINE)
                .expect("couldn't split string");
            first_line.chars().count()
        }
        fn find_height(maze: &str) -> usize {
            //maze.lines()
            maze.split(Symbol::END_OF_LINE)
                .filter(|&line| line.is_empty() == false) // don't count any empty lines (for example if an editor adds one after)
                .count()
        }

        pub fn save_to_file(&self, path: &str) -> Result<()> {
            let str = self.grid.to_string();
            file_io::write_to_path(path, str.as_bytes())?;
            Ok(())
        }

        pub fn load_from_file(path: &str) -> Result<Self> {
            let maze: String = file_io::read_file_to_string(path)?;

            let height = Self::find_height(&maze);
            let width = Self::find_width(&maze);

            let mut new_maze = MazeOld::new_empty(width, height);

            //let rows = maze.lines();

            for (y, row) in maze.lines().enumerate() {
                for (x, entry) in row.chars().enumerate() {
                    new_maze.grid.set((x, y), entry);
                }
            }

            Ok(new_maze)
        }
    }

    impl MazeOld {
        fn assert_within_bounds(&self, coord: Coord) {
            assert!(coord.0 < self.width && coord.1 < self.height);
        }

        pub fn set_symbol(&mut self, coord: Coord, symbol: Symbol) {
            //self.assert_within_bounds(coord);
            //let (x, y) = coord;

            self.grid.set(coord, symbol);
        }

        pub fn read_symbol(&self, coord: Coord) -> Symbol {
            self.grid.get(coord).clone()
        }

        pub fn coord_at_index(&self, index: usize) -> Coord {
            let x = index % self.width;
            let y = index / self.width;
            (x, y)
        }

        pub fn is_free(&self, coord: Coord) -> bool {
            self.read_symbol(coord) == Symbol::FREE
        }

        pub fn is_blocked(&self, coord: Coord) -> bool {
            self.read_symbol(coord) == Symbol::BLOCKED
        }

        //pub fn get_blocked_coords(&self) -> Vec<Coord> {
        //    self.grid
        //        .data
        //        .iter()
        //        .enumerate()
        //        .filter_map(|(i, &symbol)| {
        //            if symbol == Symbol::BLOCKED {
        //                let coord = self.coord_at_index(i);
        //                Some(coord)
        //            } else {
        //                None
        //            }
        //        })
        //        .collect::<Vec<Coord>>()
        //}
    }

    #[test]
    fn testy_yeah() {
        let mut g = MazeOld::load_from_file("saves/new_save.txt").expect("failed to load grid");
        g.set_symbol((8, 3), Symbol::PLAYER_SPAWN);
        println!("{}", g);

        println!("symbol at (4, 2) is: {}", g.read_symbol((4, 2)));

        g.save_to_file("saves/new_save.txt").unwrap();

        let maze = MazeOld::new_empty(2, 3);
        maze.save_to_file("saves/text.txt")
            .expect("failed to save to file");

        println!("last maze: {}", maze);
    }
}
