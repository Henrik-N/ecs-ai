#![allow(unused)]

mod walls_builder;
mod input;
mod resources_and_components;
mod grid;
mod game_assets;
mod physics;
mod movement;
mod util;

use resources_and_components::*;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use bevy::utils::HashSet;
use crate::game_assets::{Mats};


mod settings {
    use bevy::prelude::*;
    use bevy::window::WindowMode;

    const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
    pub const WINDOW_WIDTH: f32 = 1200.;
    pub const WINDOW_HEIGHT: f32 = 700.;

    pub struct AppSettings;

    impl Plugin for AppSettings {
        fn build(&self, app: &mut AppBuilder) {
            app
                .insert_resource(ClearColor(CLEAR_COLOR))
                .insert_resource(WindowDescriptor {
                    width: WINDOW_WIDTH,
                    height: WINDOW_HEIGHT,
                    title: "Yeet".to_owned(),
                    vsync: false,
                    resizable: false,
                    decorations: true,
                    cursor_visible: true,
                    cursor_locked: false,
                    ..Default::default()
                })
            ;
        }
    }
}

fn main() {
    App::build()
        .add_plugin(settings::AppSettings)
        .add_plugins(DefaultPlugins)
        .add_plugin(game_assets::GameAssets)
        .add_startup_system(setup.system())
        .add_plugin(input::PlayerInputPlugin)
        .insert_resource(BlockedCoords::default())
        .add_plugin(walls_builder::WallsBuilderPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(physics::PhysicsPlugin)

        .run();
}

#[derive(Debug, Default)]
struct EnemyTag;


fn setup(
    mut cmd: Commands,
    mut mats: Res<Mats>,
) {

    // add camera
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmd.spawn_bundle(UiCameraBundle::default());



    let sprite = grid::square_sprite();

    let mat = mats.get("blue");
    let spawn_pos = Vec2::new(0., -215.);

    let grid_coord: GridCoord =
        grid::get_xy_coords_from_screen_space_position(&spawn_pos).into();


    // spawn player entity
    let player = cmd.spawn_bundle(SpriteBundle {
        material: mat,
        transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.),
        sprite: sprite.clone(),
        ..Default::default()
    })

        .insert(SpriteCollider::Dynamic)
        .insert(grid_coord)
        .insert(Velocity::default())
        .insert(Player {movement_speed: 500.})
        .id()
        ;



    let start_pos = Vec2::new(0., -12.);
    let mat = mats.get("red");


    let grid_coord: GridCoord =
        grid::get_xy_coords_from_screen_space_position(&start_pos).into();

    // spawn enemy entity
    let enemy = cmd.spawn_bundle(SpriteBundle {
        material: mat,
        transform: Transform::from_xyz(start_pos.x, start_pos.y, 0.),
        sprite: sprite.clone(),
        ..Default::default()
    })
        .insert(SpriteCollider::Dynamic)
        .insert(grid_coord)
        .insert(Velocity::default())
        .insert(EnemyTag::default())
        .id()
        ;



    println!("player: {:?}, enemy: {:?}", player, enemy);
}
