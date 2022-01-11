mod application;
mod battle;
mod game_assets;
mod grid_plugin;
mod input;
mod maze;
mod movement;
mod pathfinder;
mod resources_and_components;
mod util;

use resources_and_components::*;
use std::borrow::BorrowMut;

use crate::grid_plugin::GridCoord;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::utils::HashSet;
use bevy::{
    log,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::new()
        .add_plugin(application::Application)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_entities)
        .add_plugin(input::PlayerInputPlugin)
        .add_plugin(maze::MazePlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(movement::PhysicsPlugin)
        .add_plugin(battle::BattlePlugin)
        .run();
}

fn setup_entities(mut cmd: Commands) {
    Camera2D::spawn(&mut cmd);
}

use crate::maze::MazeResource;
use entities::*;

mod entities {
    use crate::movement::MovementSpeed;
    use crate::{grid_plugin, movement, GridCoord, MazeResource, SpriteCollider, Velocity};
    use bevy::log;
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Camera2D;

    impl Camera2D {
        pub fn spawn(cmd: &mut Commands) {
            cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
            cmd.spawn_bundle(UiCameraBundle::default());
        }
    }

    #[derive(Component, Default)]
    pub struct Player;

    impl Player {
        pub(super) fn spawn(cmd: &mut Commands, spawn_pos: Vec2) -> Entity {
            let color = Color::Rgba {
                red: 0.5,
                green: 0.5,
                blue: 1.0,
                alpha: 1.0,
            };

            // spawn player entity
            cmd.spawn_bundle(SpriteBundle {
                //material: mat,
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.),
                sprite: grid_plugin::square_sprite(color),
                ..Default::default()
            })
            .insert(SpriteCollider::Dynamic)
            .insert(Velocity::default())
            .insert(MovementSpeed(500.))
            .insert(Self::default())
            .insert(movement::Collider::Player)
            .id()
        }
    }

    #[derive(Debug, Default, Component)]
    pub struct Enemy;

    impl Enemy {
        pub(super) fn spawn(cmd: &mut Commands, spawn_pos: Vec2) -> Entity {
            // spawn enemy entity
            log::trace!("spawning enemy");
            cmd.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.),
                sprite: grid_plugin::square_sprite(Color::RED),
                ..Default::default()
            })
            .insert(SpriteCollider::Dynamic)
            //.insert(grid_coord)
            .insert(Velocity::default())
            .insert(MovementSpeed(200.))
            .insert(Self::default())
            .insert(movement::Collider::Enemy)
            .id()
        }
    }
}
