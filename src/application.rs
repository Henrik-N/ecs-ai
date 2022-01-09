use bevy::prelude::*;
use bevy::window::WindowMode;

const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const WINDOW_WIDTH: f32 = 1200.;
pub const WINDOW_HEIGHT: f32 = 700.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    BuildMap,
    PlayGame,
    GameOver,
}

pub struct Application;

impl Plugin for Application {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::BuildMap)
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
            });
    }
}
