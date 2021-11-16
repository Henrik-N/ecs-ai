#![allow(unused)]

mod walls_builder;
mod input;
mod resources_and_components;

use resources_and_components::*;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

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


mod ai {
    use bevy::prelude::*;
    use crate::resources_and_components::Movement;
    use crate::SQUARE_SIDE_SIZE;

    /// --------------------------
    pub struct AIPlugin;

    impl Plugin for AIPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.
                add_startup_system(setup.system())
            ;

        }
    }


    struct TargetCoordinate((u32, u32));


    #[derive(Bundle)]
    struct EnemyBundle {
        sprite_bundle: SpriteBundle,
        movement: Movement,
        target: Option<TargetCoordinate>,
    }



    fn setup(
        mut cmd: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>) {


        let square = Sprite::new(Vec2::new(SQUARE_SIDE_SIZE, SQUARE_SIDE_SIZE));
        let ai_mat = materials.add(Color::rgb(1., 0., 0.).into());


        cmd.spawn_bundle(SpriteBundle {
            material: ai_mat,
            transform: Transform::from_xyz(0., -12., 0.),
            sprite: square.clone(),
            ..Default::default()
        })
            .insert(Movement { speed: 500. });
    }


    mod sound_events {
        use bevy::prelude::*;

        type ListeningEntity = Entity;
        type HeardEntity = Entity;

        #[derive(Default)]
        pub struct HearingEvents {
            data: Vec<(ListeningEntity, HeardEntity)>,
        }


        // tag components
        pub struct Listener;

        pub struct CanBeHeard;
    }
}




#[derive(Default)]
pub struct BlockedCoords(Vec<(u32, u32)>);


const SQUARE_SIDE_SIZE: f32 = 50.;

fn main() {
    App::build()
        .add_plugin(settings::AppSettings)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(input::PlayerInputPlugin)
        .add_plugin(ai::AIPlugin)
        .insert_resource(BlockedCoords::default())
        .add_plugin(walls_builder::WallsBuilder)
        .add_system_set(SystemSet::new()
            .after(input::PlayerInputPlugin::DEPENDENCY)
            .with_system(player_movement_system.system())
        )
        .run();
}


fn player_movement_system(
    time: Res<Time>,
    input: Res<AxisInput>,
    mut q: Query<(&Movement, &mut Transform), With<PlayerTag>>) {
    if let Ok((movement, mut transform)) = q.single_mut() {
        let translation = input.axis * movement.speed * time.delta_seconds();

        transform.translation += Vec3::new(translation.x, translation.y, 0.);
    }
}


fn setup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>) {


    // add camera
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    cmd.spawn_bundle(UiCameraBundle::default());

    let square = Sprite::new(Vec2::new(SQUARE_SIDE_SIZE, SQUARE_SIDE_SIZE));

    // player
    cmd.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 1.).into()),
        transform: Transform::from_xyz(0., -215., 0.),
        sprite: square.clone(),
        ..Default::default()
    })
        .insert(Movement { speed: 500. })
        .insert(PlayerTag)
    ;
}



