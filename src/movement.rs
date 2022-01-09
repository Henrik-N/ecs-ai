use crate::resources_and_components::{CollidedWith, CollisionData, SpriteCollider, Velocity};
use crate::util::*;
use crate::{grid_plugin, Enemy, Player};
use anyhow::Result;
use bevy::log;
use bevy::prelude::*;
use std::fmt::{Debug, Formatter};
use std::slice::Iter;

use crate::application::GameState;
use crate::input::PlayerInputPlugin;

pub use components::*;
pub mod components {
    use derive_more::{Deref, DerefMut};

    #[derive(Deref, DerefMut)]
    pub struct MovementSpeed(
        #[deref]
        #[deref_mut]
        pub f32,
    );
}

pub(crate) struct MovementPlugin;

impl MovementPlugin {
    pub const DEPENDENCY: &'static str = "MovementPlugin";
}

const UPDATE_VELOCITY_COMPONENTS: &'static str = "update vel comps";

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<Collisions>()
            .add_system_set(
                SystemSet::on_update(GameState::PlayGame)
                    .label(UPDATE_VELOCITY_COMPONENTS)
                    .after(PlayerInputPlugin::DEPENDENCY)
                    .with_system(Self::update_player_velocity.system())
                    .with_system(Self::update_enemy_velocities.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::PlayGame)
                    .label(Self::DEPENDENCY)
                    .after(UPDATE_VELOCITY_COMPONENTS)
                    .with_system(Self::movement_system.system()),
            );
        // collisions
        //.add_system_set(SystemSet::new()
        //    .label("collisions")
        //    .after(Self::DEPENDENCY)
        //    .with_system(detect_collisions.system())
        //)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MovementPluginState {
    UpdateVelocity,
    MoveEntities,
}

impl MovementPlugin {
    fn update_player_velocity(
        time: Res<Time>,
        input: Res<AxisInput>,
        mut q: Query<(&mut Velocity, &MovementSpeed), With<Player>>,
    ) {
        if let Ok((mut vel, movement_speed)) = q.single_mut() {
            let dt = time.delta_seconds();

            let input_vel = input.axis * movement_speed.0 * dt;

            vel.velocity += Vec2::new(input_vel.x, input_vel.y);
        }
    }

    fn update_enemy_velocities(
        time: Res<Time>,
        target: Query<(&Transform), With<Player>>,
        mut enemy: Query<(&Transform, &mut Velocity, &MovementSpeed), With<Enemy>>,
    ) {
        if let Ok(player_transf) = target.single() {
            let dt = time.delta_seconds();

            let player_pos = player_transf.translation;

            for (transform, mut vel, movement_speed) in enemy.iter_mut() {
                let agent_pos = transform.translation;
                let target_dir = (player_pos - agent_pos).normalize_or_zero();

                //let x = pathfinding::path_find(&to_vec2(&agent_pos), &to_vec2(&player_pos));

                vel.velocity += to_vec2(&target_dir) * movement_speed.0 * dt.clone();
            }
        }
    }

    fn movement_system(
        mut q: Query<(
            &mut Transform,
            &mut Velocity,
            Option<&mut GridCoord>,
            Option<&SpriteCollider>,
        )>,
    ) {
        for (mut transform, mut vel, mut grid_coord, sprite_collider) in q.iter_mut() {
            let current_pos = to_vec2(&transform.translation);
            let move_delta = vel.velocity; //* dt.clone();
            let target_pos = current_pos + move_delta;

            transform.translation = to_vec3(&target_pos);

            vel.velocity = Vec2::new(0., 0.);

            // update the grid coordinate on the grid component
            // this one belongs to if it has one
            if let Some(mut gc) = grid_coord {
                let target_pos = grid_plugin::block_position_to_screen_space_position(&target_pos);
                let xy_coords = grid_plugin::get_xy_coords_from_screen_space_position(&target_pos);
                *gc = xy_coords.into();
            }

            //if let Some(sprite_collider) = sprite_collider {
            //    if *sprite_collider != SpriteCollider::Static {
            //
            //    }
            //}
        }
    }
}

fn handle_collision_events(mut collision_events: EventReader<Collisions>) {
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
        app.add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    bevy::core::FixedTimestep::step(0.5).with_label(FIXED_UPDATE_LABEL),
                )
                .with_system(fixed_update.system().chain(detect_collisions.system())),
        );
    }
}

use crate::grid_plugin::{GridCoord, SQUARE_SIDE_SIZE};
use crate::input::{AxisInput, InputVelocity};
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::utils::tracing::Instrument;

#[derive(Debug)]
pub struct Collisions(Vec<CollisionData>);

#[derive(Clone)]
struct FixedDeltaTime {
    fixed_dt: f64,
    alpha: f64,
    one_minus_alpha: f64,
}

fn fixed_update(
    mut last_time: Local<f64>,
    mut yeet: Local<f64>,
    time: Res<Time>,
    fixed_timesteps: Res<bevy::core::FixedTimesteps>,
) -> FixedDeltaTime {
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
    let collisions: Vec<(Entity, Collisions)> = q
        .iter()
        .filter_map(
            |(entity, transform, sprite, collider)| -> Option<(Entity, Collisions)> {
                // static colliders won't be affected by the collision and thus don't need
                //  to calculate data about it
                if *collider == SpriteCollider::Static {
                    return None;
                }

                let pos = transform.translation;

                let collisions_data = q
                    .iter()
                    .filter_map(
                        |(entity1, transform1, sprite1, collider1)| -> Option<CollisionData> {
                            let pos1 = transform1.translation;
                            let collision = collide(pos, sprite.size, pos1, sprite1.size);

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
                        },
                    )
                    .collect::<Vec<CollisionData>>();

                // send a collision event if collided with anything
                if !collisions_data.is_empty() {
                    Some((entity, Collisions(collisions_data)))
                } else {
                    None
                }
            },
        )
        .collect();

    adjust_from_collision
        .iter_mut()
        .for_each(|(entity, mut velocity, c)| {
            // get collision data for this entity, if any
            let col_data: Option<&(Entity, Collisions)> =
                collisions.iter().find(|(e, _)| e.id() == entity.id());

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

                    let vel =
                        Vec2::new(SQUARE_SIDE_SIZE / 2., SQUARE_SIDE_SIZE / 2.) - collision.offset;
                    velocity.velocity = vel * (alpha as f32)
                        + velocity.previous_velocity * (one_minus_alpha as f32);
                }
            }

            velocity.previous_velocity = velocity.velocity;
        });
}
