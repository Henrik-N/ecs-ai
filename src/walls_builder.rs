use crate::BlockedCoords;

use std::io::Read;
use bevy::prelude::*;
use crate::resources_and_components::*;
use super::settings;
use super::SQUARE_SIDE_SIZE;


#[derive(Default)]
// tag
pub struct PreviewWallBlock {
    pub enabled: bool,
}


pub struct WallsBuilder;

impl Plugin for WallsBuilder {
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
    let square = square_sprite();

    // preview block (shown when hovering around with the mouse)
    cmd.spawn_bundle(SpriteBundle {
        material: wall_mat,
        transform: Transform::from_xyz(0., 0., 0.),
        sprite: square,
        ..Default::default()
    })
        .insert(PreviewWallBlock { enabled: true });
}


pub mod grid {
    use bevy::prelude::*;
    use crate::SQUARE_SIDE_SIZE;

    pub fn get_xy_coords(pos: &Vec2) -> (u32, u32) {
        ((pos.x / SQUARE_SIDE_SIZE) as u32,
         (pos.y / SQUARE_SIDE_SIZE) as u32)
    }

    pub fn get_aligned_pos_from_coords(xy_cords: &(u32, u32), sprite: &Sprite) -> Vec2 {
        let x_coord = xy_cords.0 as f32 * SQUARE_SIDE_SIZE;
        let y_coord = xy_cords.1 as f32 * SQUARE_SIDE_SIZE;

        Vec2::new(x_coord as f32, y_coord as f32)
            - Vec2::new(
            crate::settings::WINDOW_WIDTH / 2.,
            crate::settings::WINDOW_HEIGHT / 2.)
            + Vec2::new(sprite.size.x / 2., sprite.size.y / 2.)
    }

}
use grid::*;


fn preview_wall_block_mouse_follow(mouse_pos: Res<MousePos>, mut q: Query<(&mut Transform, &Sprite, &PreviewWallBlock)>) {
    if let Ok((mut transform, sprite, wall_block)) = q.single_mut() {
        if !wall_block.enabled {
            return;
        }

        let mouse_pos: Vec2 = mouse_pos.get();
        let xy_coords = get_xy_coords(&mouse_pos);
        let aligned_cords = get_aligned_pos_from_coords(&xy_coords, &sprite);

        let translation = Vec3::new(aligned_cords.x, aligned_cords.y, 0.);

        transform.translation = translation;
    }
}


fn construct_wall_on_left_mouse_click(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
    mut blocked_coordinates: ResMut<BlockedCoords>) {


    let construct = mouse_button_input.pressed(MouseButton::Left);

    if construct {
        let xy_coords = get_xy_coords(&mouse_pos.get());

        // only spawn new wall there's not already a wall at this location
        if blocked_coordinates.0.contains(&xy_coords) {
            return;
        }

        blocked_coordinates.0.push(xy_coords);

        update_save_file(blocked_coordinates);

        let square_sprite = square_sprite();
        let wall_mat = materials.add(Color::rgb(0.8, 0.8, 0.8).into());

        let pos = get_aligned_pos_from_coords(&xy_coords, &square_sprite);

        cmd.spawn_bundle(SpriteBundle {
            material: wall_mat,
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.)),
            sprite: square_sprite,
            ..Default::default()
        });
    }
}

fn square_sprite() -> Sprite {
    Sprite::new(Vec2::new(SQUARE_SIDE_SIZE, SQUARE_SIDE_SIZE))
}


const SAVE_FILE_PATH: &'static str = "saves/save.txt";

fn update_save_file(blocked_coordinates: ResMut<BlockedCoords>) {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    let mut file_contents = String::new();

    blocked_coordinates.0.iter().for_each(|(x, y)| {
        file_contents += &format!("{},{} ", x, y);
    });

    let path = Path::new(SAVE_FILE_PATH);
    let mut file = File::create(path).expect("couldn't create file");

    file.write_all(file_contents.as_bytes());
}

fn read_save_file_on_middle_mouse_click(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

        for c in coords {
            let c = c.to_owned();
            let (x, y) = c.split_once(",").expect("coudln't split");

            let x = x.parse::<u32>().expect("couldn't parse");
            let y = y.parse::<u32>().expect("couldn't parse");

            blocked_coordinates.0.push((x, y))
        }

        let square_sprite = square_sprite();
        let wall_mat = materials.add(Color::rgb(0.8, 0.8, 0.8).into());

        for block in blocked_coordinates.0.iter() {
            let pos = get_aligned_pos_from_coords(&block, &square_sprite);

            cmd.spawn_bundle(SpriteBundle {
                material: wall_mat.clone(),
                transform: Transform::from_translation(
                    Vec3::new(pos.x, pos.y, 0.)),
                sprite: square_sprite.clone(),
                ..Default::default()
            });
        }
    }
}
