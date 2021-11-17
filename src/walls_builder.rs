use std::borrow::BorrowMut;
use crate::BlockedCoords;

use std::io::Read;
use bevy::prelude::*;
use crate::game_assets::Mats;
use crate::resources_and_components::*;
use super::settings;
use crate::grid;
use crate::grid::get_xy_coords_from_screen_space_position;

#[derive(Default)]
// tag
pub struct PreviewWallBlock {
    pub enabled: bool,
}




pub struct WallsBuilderPlugin;

impl Plugin for WallsBuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system_set(SystemSet::new()
                .after(crate::input::PlayerInputPlugin::DEPENDENCY)
                .with_system(preview_wall_block_mouse_follow.system())
                .with_system(construct_wall_on_left_mouse_click.system())
                .with_system(read_save_file_on_middle_mouse_click.system())
            );
    }
}

fn setup(mut cmd: Commands,
         mut materials: ResMut<Assets<ColorMaterial>>) {
    // wall
    let wall_mat = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let square = grid::square_sprite();

    // preview block (shown when hovering around with the mouse)
    cmd.spawn_bundle(SpriteBundle {
        material: wall_mat,
        transform: Transform::from_xyz(0., 0., 0.),
        sprite: square,
        ..Default::default()
    })
        .insert(PreviewWallBlock { enabled: true });
}

fn preview_wall_block_mouse_follow(mouse_pos: Res<MousePos>, mut q: Query<(&mut Transform, &Sprite, &PreviewWallBlock)>) {
    if let Ok((mut transform, sprite, wall_block)) = q.single_mut() {
        if !wall_block.enabled {
            return;
        }

        let mouse_pos: Vec2 = mouse_pos.get();
        let xy_coords = grid::get_xy_coords_from_screen_space_position(&mouse_pos).into();
        let aligned_cords = grid::get_aligned_pos_from_coords(&xy_coords);

        let translation = Vec3::new(aligned_cords.x, aligned_cords.y, 0.);

        transform.translation = translation;
    }
}

fn construct_wall_on_left_mouse_click(
    mut cmd: Commands,
    mats: Res<Mats>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
    mut blocked_coordinates: ResMut<BlockedCoords>) {
    let (construct, destroy) =
        (mouse_button_input.pressed(MouseButton::Left),
         mouse_button_input.pressed(MouseButton::Right));

    if construct {
        let xy_coords = grid::get_xy_coords_from_screen_space_position(&mouse_pos.get()).into();

        // only add if there isn't one already at that location
        if !blocked_coordinates.0.contains(&xy_coords) {
            blocked_coordinates.0.push(xy_coords);
            update_save_file(blocked_coordinates);

            let square_sprite = grid::square_sprite();
            let material = mats.get("gray");
            let spawn_pos = grid::get_aligned_pos_from_coords(&xy_coords);

            let cmd_borrow: &mut Commands = cmd.borrow_mut();

            spawn_wall(
                &mut cmd,
                SpawnWallData {
                    spawn_pos,
                    spawn_coords: xy_coords.into(),
                    sprite: square_sprite,
                    material,
                });
        }
    }
}


struct SpawnWallData {
    spawn_pos: Vec2,
    spawn_coords: GridCoord,
    sprite: Sprite,
    material: Handle<ColorMaterial>,
}

/// Spawns a block with a static collider
fn spawn_wall(
    mut cmd: &mut Commands,
    data: SpawnWallData) {
    cmd
        .spawn_bundle(SpriteBundle {
            material: data.material,
            transform: Transform::from_translation(
                Vec3::new(data.spawn_pos.x, data.spawn_pos.y, 0.)),
            sprite: data.sprite.clone(),
            ..Default::default()
        })
        .insert(SpriteCollider::Static)
        .insert(data.spawn_coords)
    ;
}


const SAVE_FILE_PATH: &'static str = "saves/save.txt";

fn update_save_file(blocked_coordinates: ResMut<BlockedCoords>) {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    let mut file_contents = String::new();

    blocked_coordinates.0.iter().for_each(|xy| {
        file_contents += &format!("{},{} ", xy.x, xy.y);
    });

    let path = Path::new(SAVE_FILE_PATH);
    let mut file = File::create(path).expect("couldn't create file");

    file.write_all(file_contents.as_bytes());
}

fn read_save_file_on_middle_mouse_click(
    mut cmd: Commands,
    mats: Res<Mats>,
    input: Res<Input<MouseButton>>,
    mut blocked_coordinates: ResMut<BlockedCoords>,
) {
    if input.just_pressed(MouseButton::Middle) {
        use std::fs::File;
        use std::io::Read;
        use std::io::{BufReader, BufRead};

        use std::path::Path;

        let file = File::open(SAVE_FILE_PATH).expect("couldn't open file");

        let mut reader = BufReader::new(file);

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer);

        // remove trailing whitespace
        buffer = buffer.trim().to_owned();

        let coords = buffer.split_whitespace();

        // find which coordinates to spawn new blocks at
        let coords_to_spawn_at = coords.into_iter().filter_map(|coord: &str| {
            let coord = coord.to_owned();
            let (x, y): (&str, &str) = coord.split_once(",").expect("coudln't split");

            let xy: GridCoord = (x.parse::<u32>().expect("couldn't parse"),
                                 y.parse::<u32>().expect("couldn't parse"))
                .into();

            if !blocked_coordinates.0.contains(&xy) {
                Some(xy)
            } else {
                None
            }
        }).collect::<Vec<GridCoord>>();


        // spawn new blocks
        let square_sprite = grid::square_sprite();
        let wall_mat = mats.get("gray");

        coords_to_spawn_at.into_iter().for_each(|coord| {
            blocked_coordinates.0.push(coord.clone());

            let pos = grid::get_aligned_pos_from_coords(&coord);

            spawn_wall(
                &mut cmd,
                SpawnWallData {
                    spawn_pos: pos,
                    spawn_coords: coord,
                    sprite: square_sprite.clone(),
                    material: wall_mat.clone(),
                });
        });
    }
}
