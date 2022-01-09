use bevy::{prelude::*, window::CursorMoved};

use crate::resources_and_components::*;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};

use crate::Player;

pub use resources::*;
mod resources {
    use bevy::prelude::*;
    use derive_more::{Deref, DerefMut};

    #[derive(Debug, Default, Deref, DerefMut)]
    pub struct AxisInput {
        #[deref]
        #[deref_mut]
        pub axis: Vec2,
    }

    impl AxisInput {
        pub(super) fn reset(&mut self) {
            self.axis = Vec2::default();
        }
    }

    #[derive(Debug, Default, Deref)]
    pub struct MousePos(#[deref] pub Vec2);
}

pub use components::*;
mod components {
    use derive_more::Deref;

    // entities with this component will apply velocity from player input
    #[derive(Deref)]
    pub struct InputVelocity(#[deref] pub f32);
}

pub use events::*;
mod events {
    use bevy::prelude::*;

    pub struct MouseLeftEvent {
        pub mouse_pos: Vec2,
        pub shift_held: bool,
    }

    pub struct MouseRightEvent {
        pub mouse_pos: Vec2,
        pub shift_held: bool,
    }
}

pub struct PlayerInputPlugin;

impl PlayerInputPlugin {
    pub const DEPENDENCY: &'static str = "player_input";
}

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AxisInput::default())
            .insert_resource(MousePos::default())
            .add_event::<MouseLeftEvent>()
            .add_event::<MouseRightEvent>()
            .add_system_set(
                SystemSet::new()
                    .label(Self::DEPENDENCY)
                    .with_system(Self::axis_input.system())
                    .with_system(Self::process_mouse_states.system()),
            );
    }
}

impl PlayerInputPlugin {
    fn process_mouse_states(
        mut cursor_moved_events: EventReader<CursorMoved>,
        mut mouse_loc: ResMut<MousePos>,
        mouse_button_input: Res<Input<MouseButton>>,
        keyboard: Res<Input<KeyCode>>,

        mut mouse_left_event: EventWriter<MouseLeftEvent>,

        mut mouse_right_event: EventWriter<MouseRightEvent>,
    ) {
        for event in cursor_moved_events.iter() {
            let pos = event.position;

            mouse_loc.0 = Vec2::new(pos.x.clone(), pos.y.clone());
        }

        let shift_held = keyboard.pressed(KeyCode::LShift) || keyboard.pressed(KeyCode::RShift);

        if mouse_button_input.pressed(MouseButton::Left) {
            mouse_left_event.send(MouseLeftEvent {
                mouse_pos: mouse_loc.0,
                shift_held,
            })
        }

        if mouse_button_input.pressed(MouseButton::Right) {
            mouse_right_event.send(MouseRightEvent {
                mouse_pos: mouse_loc.0,
                shift_held,
            })
        }
    }

    fn axis_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut axis_input: ResMut<AxisInput>,
        mut q: Query<(&mut Velocity), With<Player>>,
    ) {
        axis_input.reset();

        if keyboard_input.pressed(KeyCode::A) {
            axis_input.x -= 1.;
        }

        if keyboard_input.pressed(KeyCode::D) {
            axis_input.x += 1.;
        }

        if keyboard_input.pressed(KeyCode::W) {
            axis_input.y += 1.;
        }

        if keyboard_input.pressed(KeyCode::S) {
            axis_input.y -= 1.;
        }
    }
}
