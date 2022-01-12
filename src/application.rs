use bevy::core::FixedTimestep;
use bevy::ecs::schedule::{IntoRunCriteria, ShouldRun};
use bevy::ecs::system::ChainSystem;
use bevy::prelude::*;
use bevy::window::WindowMode;

const CLEAR_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const WINDOW_WIDTH: f32 = 1200.;
pub const WINDOW_HEIGHT: f32 = 700.;


pub const TIME_STEP: f64 = 1.0 / 60.0;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    BuildMap,
    PlayGame,
    GameOver,
}

/// RunCriteria that is both FixedTimeStep criteria and GameState::PlayGame
#[macro_export]
macro_rules! fixed_time_step_dependant_state {
    ($state:expr) => {
         bevy::core::FixedTimestep::step(crate::application::TIME_STEP).chain(
                            |In(input): In<ShouldRun>, state: Res<State<GameState>>| {
                                if *state.current() == $state {
                                    input
                                } else {
                                    ShouldRun::No
                                }
                            },
                        )
    };
}


pub struct Application;

impl Plugin for Application {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::BuildMap)
            .insert_resource(ClearColor(CLEAR_COLOR))
            .insert_resource(WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: "Yeet".to_owned(),
                vsync: true,
                resizable: false,
                decorations: true,
                cursor_visible: true,
                cursor_locked: false,
                ..Default::default()
            });
    }
}
