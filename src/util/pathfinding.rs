use crate::grid_plugin;
use crate::grid_plugin::GridCoord;
use bevy::ecs::prelude::Res;
use bevy::prelude::Vec2;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    parent_node: GridCoord,
    coord: GridCoord,
    g_cost: u32,
    // distance between current and start
    h_cost: u32, // estimated distance from current node to end node
}

impl Node {
    fn init_start(start: GridCoord, end: GridCoord) -> Self {
        let g_cost = 0;

        Self {
            parent_node: Default::default(),
            coord: start,
            g_cost: 0,
            h_cost: Self::h_cost(&start, &end),
        }
    }

    fn h_cost(from: &GridCoord, end: &GridCoord) -> u32 {
        end.x * end.x + end.y * end.y + from.x * from.x + from.y * from.y
    }

    fn f_cost(&self) -> u32 {
        self.g_cost + self.h_cost
    }

    //fn find_adjacent_nodes(&self,
    //                       open_set: &mut BinaryHeap<Node>,
    //                       closed_set: &BinaryHeap<Node>,
    //                       blocked_nodes: Res<BlockedCoords>) {
    //    // if not walkable or in closed set, ignore
    //    let dirs: [GridCoord; 4] = [
    //        (self.coord.x - 1, self.coord.y).into(), // left,
    //        (self.coord.x + 1, self.coord.y).into(), // right,
    //        (self.coord.x, self.coord.y + 1).into(), // up,
    //        (self.coord.x, self.coord.y - 1).into() // down,
    //    ];

    //    let walkable_adjacent_nodes = dirs
    //        .filter(|coord| {
    //            // within the grid's bounds
    //            grid::is_coordinate_within_borders(&coord) &&
    //                // not closed
    //                !closed_set.iter().any(|node| node.coord == coord) &&
    //                // walkable
    //                !blocked_nodes.0.contains(coord)
    //        }).collect::<Vec<Node>>();

    //    walkable_adjacent_nodes.into_iter().for_each(|adj_node| {
    //        if !open_set.contains(adj_node) {
    //            open_set.push(adj_node.clone());
    //        }

    //    });
    //}

    fn adjacent_coords(coord: &GridCoord) {}
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost() == other.f_cost()
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.f_cost() < other.f_cost() {
            Some(Ordering::Less)
        } else if self.f_cost() > other.f_cost() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

pub fn path_find(from_pos: &Vec2, to_pos: &Vec2) {
    let from: Vec2 = grid_plugin::block_position_to_screen_space_position(&from_pos);
    let to: Vec2 = grid_plugin::block_position_to_screen_space_position(&to_pos);

    let from_coord: GridCoord = grid_plugin::get_xy_coords_from_screen_space_position(&from).into();
    let to_coord: GridCoord = grid_plugin::get_xy_coords_from_screen_space_position(&to).into();

    let mut open_set = BinaryHeap::new();

    open_set.push(Reverse(
        // to make it a min-heap
        Node::init_start(from_coord, to_coord),
    ));

    let mut closed_set = BinaryHeap::new();

    while !open_set.is_empty() {
        // get lowest_f_cost_node and move it to the closed_set
        let lowest_f_cost: Reverse<Node> = open_set.pop().unwrap();
        closed_set.push(Reverse(lowest_f_cost.clone()));

        // TODO: find adjacent nodes
        let lowest_f_cost_coords = lowest_f_cost.0.coord;

        open_set.clear();
    }

    println!("From coords: {:?}", from_coord);
    println!("To coords: {:?}", to_coord);
    println!();
}
