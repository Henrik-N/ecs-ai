use crate::application::GameState;
use crate::input::MouseLeftEvent;
use crate::movement::{MovementPlugin, MovementSpeed};
use crate::{movement, MazeResource, Player, Velocity};
use bevy::prelude::*;
use std::default::Default;

#[derive(Default)]
pub struct Bullets {
    bullets: Vec<Entity>,
    last_ball_used: usize,
}

impl Bullets {
    //fn init(&mut self, cmd: &mut Commands, bullet_count: usize, maze: &MazeResource) {
    //    self.bullets.clear();
    //    self.last_ball_used = 0;

    //    let sprite = maze.square_sprite(Color::LIME_GREEN);
    //    self.bullets = (0..bullet_count)
    //        .map(|_| {
    //            Bullet::spawn(cmd, sprite.clone())
    //        }).collect::<Vec<_>>();
    //}

    fn next_ball(&mut self) -> Entity {
        self.last_ball_used = (self.last_ball_used + 1) % self.bullets.len();
        self.bullets[self.last_ball_used]
    }
}

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app
            //.insert_resource(Bullets::default())
            //.add_system_set(SystemSet::on_enter(GameState::PlayGame).with_system(setup_system))
            .add_system_set(SystemSet::on_update(GameState::PlayGame).with_system(fire_system))
            .add_system_set(SystemSet::on_exit(GameState::PlayGame).with_system(shutdown_system));
    }
}

#[derive(Component)]
struct Bullet;

impl Bullet {
    pub fn spawn(cmd: &mut Commands, sprite: Sprite, velocity: Velocity) -> Entity {
        cmd.spawn_bundle(SpriteBundle {
            sprite,
            ..Default::default()
        })
        .insert(Self)
        .insert(velocity)
        .insert(MovementSpeed(500.))
        .insert(movement::Collider::Bullet)
        .id()
    }
}

fn shutdown_system(mut cmd: Commands, mut bullets: ResMut<Bullets>, maze: Res<MazeResource>) {}

fn fire_system(
    mut cmd: Commands,
    mut maze: Res<MazeResource>,
    mut mb0_events: EventReader<MouseLeftEvent>,
    q: Query<&Transform, With<Player>>,
) {
    for mouse_event in mb0_events.iter() {
        let player_pos = q.single().translation;
        let mouse_pos = mouse_event.mouse_pos - (Vec2::from(maze.screen_dimensions) / 2.);
        let dir = ((mouse_pos) - player_pos.truncate()).normalize();
        let speed = 50.;

        let side_size = maze.square_block_side_length / 4.;

        let sprite = Sprite {
            custom_size: Some(Vec2::new(side_size, side_size)),
            ..Sprite::from(maze.square_sprite(Color::LIME_GREEN))
        };

        let bullet = cmd
            .spawn_bundle(SpriteBundle {
                sprite,
                ..Default::default()
            })
            .insert(Bullet)
            .insert(Velocity {
                velocity: dir * speed,
                ..Default::default()
            })
            .insert(Transform::from_xyz(player_pos.x, player_pos.y, 0.))
            .insert(movement::Collider::Bullet)
            .id();
    }
}
