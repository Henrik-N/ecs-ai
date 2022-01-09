#![allow(unused)]

mod application;
mod game_assets;
mod game_state;
mod grid_plugin;
mod input;
mod maze;
mod movement;
mod physics;
mod resources_and_components;
mod util;

use resources_and_components::*;
use std::borrow::BorrowMut;

use crate::game_assets::Mats;
use crate::grid_plugin::GridCoord;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::utils::HashSet;
use bevy::{
    log,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::build()
        .add_plugin(application::Application)
        .add_plugins(DefaultPlugins)
        // diagnostics
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(LogDiagnosticsPlugin::default())
        // disagnostics end
        .add_plugin(game_assets::GameAssets)
        .add_startup_system(setup_entities.system())
        .add_plugin(input::PlayerInputPlugin)
        .add_plugin(maze::MazePlugin)
        .add_plugin(grid_plugin::GridPlugin)
        //.add_plugin(walls_builder::WallsBuilderPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(movement::PhysicsPlugin)
        .run();
}

fn setup_entities(mut cmd: Commands, mut mats: Res<Mats>) {
    Camera2D::spawn(&mut cmd);
    Player::spawn(&mut cmd, &mut mats);
    Enemy::spawn(&mut cmd, &mut mats);
}





use entities::*;
use crate::maze::MazeResource;

mod entities {
    use crate::movement::MovementSpeed;
    use crate::{grid_plugin, GridCoord, Mats, MazeResource, SpriteCollider, Velocity};
    use bevy::log;
    use bevy::prelude::*;

    pub struct Camera2D;

    impl Camera2D {
        pub fn spawn(cmd: &mut Commands) {
            cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
            cmd.spawn_bundle(UiCameraBundle::default());
        }
    }

    pub struct Player;

    impl Player {
        //pub fn sprite_bundle(mats: &Res<Mats>, maze_resource: &Res<MazeResource>) -> SpriteBundle {
        //    let sprite = maze_resource.square_sprite();

        //    SpriteBundle {
        //        material: mats.get("blue"),
        //        transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.),
        //        sprite,
        //        ..Default::default()
        //    }
        //}

        pub(super) fn spawn(cmd: &mut Commands, mats: &mut Res<Mats>) {
            let sprite = grid_plugin::square_sprite();

            let mat = mats.get("blue");
            let spawn_pos = Vec2::new(0., -215.);

            let grid_coord: GridCoord =
                grid_plugin::get_xy_coords_from_screen_space_position(&spawn_pos).into();

            // spawn player entity
            let player = cmd
                .spawn_bundle(SpriteBundle {
                    material: mat,
                    transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.),
                    sprite: sprite.clone(),
                    ..Default::default()
                })
                .insert(SpriteCollider::Dynamic)
                .insert(grid_coord)
                .insert(Velocity::default())
                .insert(MovementSpeed(500.))
                .insert(Self)
                //.insert(Self {movement_speed: 500.})
                .id();

            log::trace!("player spawned: {:?}", player);
        }
    }

    #[derive(Debug, Default)]
    pub struct Enemy;

    impl Enemy {
        pub(super) fn spawn(cmd: &mut Commands, mats: &mut Res<Mats>) {
            let sprite = grid_plugin::square_sprite();

            let start_pos = Vec2::new(0., -12.);
            let mat = mats.get("red");

            let grid_coord: GridCoord =
                grid_plugin::get_xy_coords_from_screen_space_position(&start_pos).into();

            // spawn enemy entity
            let enemy = cmd
                .spawn_bundle(SpriteBundle {
                    material: mat,
                    transform: Transform::from_xyz(start_pos.x, start_pos.y, 0.),
                    sprite: sprite.clone(),
                    ..Default::default()
                })
                .insert(SpriteCollider::Dynamic)
                .insert(grid_coord)
                .insert(Velocity::default())
                .insert(MovementSpeed(200.))
                .insert(Self::default())
                .id();

            log::trace!("enemy spawned: {:?}", enemy);
        }
    }
}
