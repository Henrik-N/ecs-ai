use std::slice::Iter;
use bevy::prelude::*;
use crate::resources_and_components::{AxisInput, Velocity, Player, GridCoord, SpriteCollider};
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
            .insert_resource(CollisionsResource::expected_max_collisions_per_frame(50))
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
            .add_system_set(SystemSet::new()
                .label("collisions")
                .after(Self::DEPENDENCY)
                .with_system(handle_collisions.system())
            )
        ;
    }
}

fn apply_player_velocity_from_input(
    input: Res<AxisInput>,
    mut q: Query<(&mut Velocity, &Player)>) {
    if let Ok((mut vel, player)) = q.single_mut() {
        let input_vel = input.axis * player.movement_speed;

        vel.0 = Vec2::new(input_vel.x, input_vel.y);
    }
}


const AI_MOVEMENT_SPEED: f32 = 200.;

fn ai_update_velocity(
    player_q: Query<&Transform, With<Player>>,
    mut agent: Query<(&Transform, &mut Velocity), Without<Player>>,
) {
    if let Ok(player_transf) = player_q.single() {
        let player_pos = player_transf.translation;


        for (transform, mut vel) in agent.iter_mut() {
            let agent_pos = transform.translation;
            let target_dir = (player_pos - agent_pos).normalize_or_zero();


            vel.0 = to_vec2(&target_dir) * AI_MOVEMENT_SPEED;
        }
    }
}


#[derive(Debug)]
struct CollisionSystemData {
    entity: Entity,
    pos: Vec2,
}

fn movement_system(
    time: Res<Time>,
    input: Res<AxisInput>,
    mut q: Query<(&mut Transform, &Velocity, Option<&mut GridCoord>)>) {
    let dt = time.delta_seconds();


    let mut msg = String::new();

    for (mut transform, vel, mut grid_coord) in q.iter_mut() {
        let current_pos = to_vec2(&transform.translation);
        let move_delta = vel.0 * dt.clone();
        let target_pos = current_pos + move_delta;

        transform.translation = to_vec3(&target_pos);


        // update the grid coordinate on the grid component
        // this one belongs to if it has one
        if let Some(mut gc) = grid_coord {
            let target_pos = grid::block_position_to_screen_space_position(&target_pos);
            let xy_coords = grid::get_xy_coords_from_screen_space_position(&target_pos);
            *gc = xy_coords.into();
        }
    };
}


use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::utils::tracing::Instrument;


#[derive(Debug)]
enum CollisionInteractionType {
    StaticWithDynamic,
    DynamicWithDynamic,
}

#[derive(Debug)]
struct CollisionInteraction {
    iteration: usize,
    interaction: CollisionInteractionType,
    collided: (Entity, Entity),
    // if static interaction, the first tuple item is the static one
    offset: Vec3, // collided.1 - collided.0
}


struct CollisionsResource {
    collisions: Vec<CollisionInteraction>,
}

impl CollisionsResource {
    fn expected_max_collisions_per_frame(count: usize) -> Self {
        Self { collisions: Vec::with_capacity(count) }
    }

    fn process(&self) -> Iter<'_, CollisionInteraction> {
        self.collisions.iter()
    }

    fn clear(&mut self) {
        self.collisions.clear()
    }
}

struct CollisionVelocityToAdd {
    velocities: Vec<(Entity, Velocity)>,
}


fn handle_collisions(
    q: Query<(Entity, &Transform, &Sprite, &SpriteCollider)>,
    mut collisions_resource: ResMut<CollisionsResource>) {
    let mut inner_start_index = 1;
    let mut possible_combos = String::new();

    let mut inner_start_index = 0;

    let collection_vec: Vec<CollisionInteraction> = Vec::new();


    //for (i, (entity, transform, sprite, collider)) in q.iter().enumerate() {

    let collisions = q.iter()
        .enumerate().map(|(i, (entity, transform, sprite, collider))| {
        let pos = transform.translation;
        inner_start_index += 1;


        let mut additional_collisions: Vec<(Entity, Velocity)> = Vec::new();

        let mut collisions1 = q.iter().enumerate().skip(inner_start_index)
            .filter_map(move |(k, (entity1, transform1, sprite1, collider1))| -> Option<(Entity, Velocity)> {
                let pos1 = transform1.translation;

                let collision = collide(
                    pos,
                    sprite.size,
                    pos1,
                    sprite1.size,
                );

                if let Some(collision) = collision {
                    let offset = pos1 - pos;

                    // if both are the same, they are both dynamic collider
                    if collider == collider1 {
                        let offset = pos1 - pos;
                        let velocity_to_add: Velocity = (offset / 2.).into();


                        let first: (Entity, Velocity) = (entity1, velocity_to_add.clone());
                        let second: (Entity, Velocity) = (entity, velocity_to_add);

                        //additional_collisions.push(first);

                        Some(second)

                    } else { // otherwise, one of them is static
                        match collider {
                            SpriteCollider::Static => {
                                let offset = pos - pos1;
                                let velocity_to_add: Velocity = offset.into();

                                Some((entity1, velocity_to_add))
                            }
                            SpriteCollider::Dynamic => {
                                let offset = pos1 - pos;
                                let velocity_to_add: Velocity = offset.into();

                                Some((entity, velocity_to_add))
                            }
                        }
                    }
                } else { // no collision
                    None
                }
            }).into_iter();
        let collisions1 = collisions1.chain(additional_collisions.into_iter());

        collisions1
    });
}

