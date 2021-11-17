use bevy::{
    prelude::*,
    window::CursorMoved,
};

use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use crate::resources_and_components::*;


pub struct PlayerInputPlugin;

impl PlayerInputPlugin {
    pub const DEPENDENCY: &'static str = "player_input";
}

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(AxisInput::default())
            .insert_resource(MousePos::default())
            .add_system_set(SystemSet::new()
                .label(Self::DEPENDENCY)
                .with_system(axis_input.system())
                .with_system(update_mouse_position.system())
            );
    }
}


fn update_mouse_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_loc: ResMut<MousePos>,
) {
    for event in cursor_moved_events.iter() {
        let pos = event.position;

        mouse_loc.0 = Vec2::new(pos.x.clone(), pos.y.clone());
    }
}


fn axis_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut axis_input: ResMut<AxisInput>,
    mut q: Query<(&mut Velocity), With<Player>>) {

    let mut axis = Vec2::default();

    if keyboard_input.pressed(KeyCode::A) {
        axis.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::D) {
        axis.x += 1.;
    }

    if keyboard_input.pressed(KeyCode::W) {
        axis.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::S) {
        axis.y -= 1.;
    }

    axis_input.axis = axis.normalize_or_zero();
}
