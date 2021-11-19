use std::fmt::{Debug, Formatter};
use std::slice::Iter;
use bevy::prelude::*;
use crate::resources_and_components::{AxisInput, Velocity, Player, GridCoord, SpriteCollider, CollisionData, CollidedWith};
use crate::grid;
use crate::util::*;
use anyhow::Result;

pub(crate) struct MovementPlugin;

impl MovementPlugin {
    pub const DEPENDENCY: &'static str = "MovementPlugin";
}

const UPDATE_VELOCITY_COMPONENTS: &'static str = "update vel comps";

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<Collisions>()

            .add_system_set(
                SystemSet::new()
                    .label(UPDATE_VELOCITY_COMPONENTS)
                    .after(crate::input::PlayerInputPlugin::DEPENDENCY)
                    .with_system(apply_player_velocity_from_input.system())
                    .with_system(ai_update_velocity.system())
            )
            .add_system_set(
                SystemSet::new()
                    .label(Self::DEPENDENCY)
                    .after(UPDATE_VELOCITY_COMPONENTS)
                    .with_system(movement_system.system())
            )
        // collisions
        //.add_system_set(SystemSet::new()
        //    .label("collisions")
        //    .after(Self::DEPENDENCY)
        //    .with_system(detect_collisions.system())
        //)
        ;
    }
}

fn apply_player_velocity_from_input(
    time: Res<Time>,
    input: Res<AxisInput>,
    mut q: Query<(&mut Velocity, &Player)>) {
    if let Ok((mut vel, player)) = q.single_mut() {
        let dt = time.delta_seconds();

        let input_vel = input.axis * player.movement_speed * dt;

        vel.velocity += Vec2::new(input_vel.x, input_vel.y);
    }
}


const AI_MOVEMENT_SPEED: f32 = 200.;

fn ai_update_velocity(
    time: Res<Time>,
    player_q: Query<(&Transform), With<Player>>,
    mut agent: Query<(&Transform, &mut Velocity), Without<Player>>,
) {
    if let Ok(player_transf) = player_q.single() {
        let dt = time.delta_seconds();

        let player_pos = player_transf.translation;


        for (transform, mut vel) in agent.iter_mut() {
            let agent_pos = transform.translation;
            let target_dir = (player_pos - agent_pos).normalize_or_zero();


            let x = pathfinding::path_find(&to_vec2(&agent_pos), &to_vec2(&player_pos));


            vel.velocity += to_vec2(&target_dir) * AI_MOVEMENT_SPEED * dt.clone();
        }
    }
}


mod pathfinding {
    use std::cmp::{Ordering, Reverse};
    use std::collections::BinaryHeap;
    use std::rc::Rc;
    use bevy::ecs::prelude::Res;
    use bevy::prelude::Vec2;
    use crate::grid;
    use crate::resources_and_components::{BlockedCoords, GridCoord};


    //impl Node {
    //    fn f_cost() {

    //    }

    //    fn update(mut self, start: GridCoord, target: GridCoord) -> Self {
    //        let from_start = self.coord - start;
    //        self.g_cost = from_start.x + from_start.y;

    //        let to_target  = target - self.coord;
    //        self.h_cost = to_target.x + to_target.y;
    //        self
    //    }

    //    fn g_cost_to_this(&self, start: GridCoord) -> u32 {
    //        let offset = self.coord - start;
    //        offset.x + offset.y
    //    }

    //}
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


        fn find_adjacent_nodes(&self,
                               open_set: &mut BinaryHeap<Node>,
                               closed_set: &BinaryHeap<Node>,
                               blocked_nodes: Res<BlockedCoords>) {
            // if not walkable or in closed set, ignore
            let dirs: [GridCoord; 4] = [
                (coord.x - 1, coord.y).into(), // left,
                (coord.x + 1, coord.y).into(), // right,
                (coord.x, coord.y + 1).into(), // up,
                (coord.x, coord.y - 1).into() // down,
            ];

            let walkable_adjacent_nodes = dirs
                .filter(|coord| {
                    // within the grid's bounds
                    grid::is_coordinate_within_borders(&coord) &&
                        // not closed
                        !closed_set.iter().any(|node| node.coord == coord) &&
                        // walkable
                        !blocked_nodes.0.contains(coord)
                }).collect::<Vec<Node>>();

            walkable_adjacent_nodes.into_iter().for_each(|adj_node| {
                if !open_set.contains(adj_node) {
                    open_set.push(adj_node.clone());
                }


            });
        }

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
        let from: Vec2 = grid::block_position_to_screen_space_position(&from_pos);
        let to: Vec2 = grid::block_position_to_screen_space_position(&to_pos);

        let from_coord: GridCoord = grid::get_xy_coords_from_screen_space_position(&from).into();
        let to_coord: GridCoord = grid::get_xy_coords_from_screen_space_position(&to).into();


        let mut open_set = BinaryHeap::new();

        open_set.push(
            Reverse( // to make it a min-heap
                     Node::init_start(from_coord, to_coord)
            ));

        let mut closed_set: BinaryHeap::new();


        while !open_set.is_empty() {
            // get lowest_f_cost_node and move it to the closed_set
            let lowest_f_cost: Reverse<Node> = open_set.pop().unwrap();
            closed_set.push(Reverse(lowest_f_cost_node.clone()));

            // find adjacent nodes
            let lowest_f_cost_coords = lowest_f_cost.0.coord;


            open_set.clear();
        }


        println!("From coords: {:?}", from_coord);
        println!("To coords: {:?}", to_coord);
        println!();
    }
}


fn movement_system(
    mut q: Query<(&mut Transform, &mut Velocity, Option<&mut GridCoord>, Option<&SpriteCollider>)>) {
    for (mut transform, mut vel, mut grid_coord, sprite_collider) in q.iter_mut() {
        let current_pos = to_vec2(&transform.translation);
        let move_delta = vel.velocity; //* dt.clone();
        let target_pos = current_pos + move_delta;

        transform.translation = to_vec3(&target_pos);

        vel.velocity = Vec2::new(0., 0.);

        // update the grid coordinate on the grid component
        // this one belongs to if it has one
        if let Some(mut gc) = grid_coord {
            let target_pos = grid::block_position_to_screen_space_position(&target_pos);
            let xy_coords = grid::get_xy_coords_from_screen_space_position(&target_pos);
            *gc = xy_coords.into();
        }

        //if let Some(sprite_collider) = sprite_collider {
        //    if *sprite_collider != SpriteCollider::Static {
        //
        //    }
        //}
    };
}


fn handle_collision_events(
    mut collision_events: EventReader<Collisions>) {
    for event in collision_events.iter() {
        //println!("Collision event!: {}", event.entity_id.id());
    }
}


const FIXED_UPDATE_LABEL: &'static str = "fixed update";

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_stage_after(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(
                        bevy::core::FixedTimestep::step(0.5)
                            .with_label(FIXED_UPDATE_LABEL),
                    )
                    .with_system(
                        fixed_update.system()
                            .chain(
                                detect_collisions.system()
                            )
                    ),
            );
    }
}


use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::utils::tracing::Instrument;
use crate::grid::SQUARE_SIDE_SIZE;


#[derive(Debug)]
pub struct Collisions(Vec<CollisionData>);

#[derive(Clone)]
struct FixedDeltaTime {
    fixed_dt: f64,
    alpha: f64,
    one_minus_alpha: f64,
}

fn fixed_update(mut last_time: Local<f64>,
                mut yeet: Local<f64>,
                time: Res<Time>,
                fixed_timesteps: Res<bevy::core::FixedTimesteps>) -> FixedDeltaTime {
    let fixed_update = time.seconds_since_startup() - *last_time;

    //println!("fixed update: {}", fixed_update);

    let fixed_timestep = fixed_timesteps.get(FIXED_UPDATE_LABEL).unwrap();
    //println!("overstep percentage: {}", fixed_timestep.overstep_percentage());
    *last_time = time.seconds_since_startup();

    *yeet = fixed_update.clone() * fixed_timestep.overstep_percentage();


    FixedDeltaTime {
        fixed_dt: fixed_update,
        alpha: fixed_timestep.overstep_percentage(),
        one_minus_alpha: 1. - fixed_timestep.overstep_percentage(),
    }
}


fn detect_collisions(
    In(fixed_dt): In<FixedDeltaTime>,
    //time: Res<Time>,
    q: Query<(Entity, &Transform, &Sprite, &SpriteCollider)>,
    mut adjust_from_collision: Query<(Entity, &mut Velocity, &SpriteCollider)>,
    mut collision_events_sender: EventWriter<Collisions>,
) {
    // get the indices that are going to move as the result of a collision
    let collisions: Vec<(Entity, Collisions)> =
        q.iter().filter_map(|(entity, transform, sprite, collider)| -> Option<(Entity, Collisions)> {
            // static colliders won't be affected by the collision and thus don't need
            //  to calculate data about it
            if *collider == SpriteCollider::Static {
                return None;
            }

            let pos = transform.translation;

            let collisions_data = q.iter()
                .filter_map(|(entity1, transform1, sprite1, collider1)|
                             -> Option<CollisionData> {
                    let pos1 = transform1.translation;
                    let collision = collide(
                        pos,
                        sprite.size,
                        pos1,
                        sprite1.size,
                    );

                    match collision {
                        None => None, // didn't collide
                        Some(collision) => {
                            let collided_with = match collider1 {
                                SpriteCollider::Static => CollidedWith::Static,
                                SpriteCollider::Dynamic => CollidedWith::Dynamic(entity1),
                            };

                            Some(CollisionData {
                                collision_side: collision,
                                collided_with,
                                offset: to_vec2(&(pos - pos1)),
                            })
                        }
                    }
                }).collect::<Vec<CollisionData>>();


            // send a collision event if collided with anything
            if !collisions_data.is_empty() {
                Some(
                    (entity, Collisions(collisions_data))
                )
            } else {
                None
            }
        }).collect();


    adjust_from_collision.iter_mut()
        .for_each(|(entity, mut velocity, c)| {
            // get collision data for this entity, if any
            let col_data: Option<&(Entity, Collisions)> = collisions.iter().find(|(e, _)| e.id() == entity.id());

            if let Some((_e, collisions)) = col_data {
                for collision in collisions.0.iter() {
                    //let dt = time.delta_seconds();
                    let FixedDeltaTime {
                        fixed_dt,
                        alpha,
                        one_minus_alpha,
                    } = fixed_dt.clone();

                    let speed = 200.;

                    let square_side = match collision.collided_with {
                        CollidedWith::Static => SQUARE_SIDE_SIZE * 2.,
                        CollidedWith::Dynamic(_) => SQUARE_SIDE_SIZE, // since the other
                        // one will move as well
                    };

                    //let square = Vec2::new(SQUARE_SIDE_SIZE / 2., SQUARE_SIDE_SIZE / 2.) * dt;

                    let dir = match collision.collision_side {
                        Collision::Left => Vec2::new(-1., 0.) * collision.offset.x,
                        Collision::Right => Vec2::new(1., 0.) * collision.offset.x,
                        Collision::Top => Vec2::new(0., 1.) * collision.offset.y,
                        Collision::Bottom => Vec2::new(0., -1.) * collision.offset.y,
                    };


                    let distance_left = SQUARE_SIDE_SIZE - dir.length();


                    let percentage_left = (SQUARE_SIDE_SIZE - collision.offset.x) / 100.;

                    let vel_to_add = dir * percentage_left;

                    let vel = velocity.velocity + -vel_to_add;
                    //- dir * (fixed_dt as f32);

                    let vel = Vec2::new(SQUARE_SIDE_SIZE / 2., SQUARE_SIDE_SIZE / 2.) - collision.offset;
                    velocity.velocity = vel * (alpha as f32)
                        + velocity.previous_velocity * (one_minus_alpha as f32);
                }
            }

            velocity.previous_velocity = velocity.velocity;
        });
}
