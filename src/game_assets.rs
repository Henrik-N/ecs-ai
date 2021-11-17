use std::collections::HashMap;
use std::ops::Deref;
use bevy::asset::HandleId;
use bevy::prelude::*;

pub struct GameAssets;
impl GameAssets {
    const DEPENDENCY: &'static str = "GameAssets";
}

impl Plugin for GameAssets {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(Mats::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, setup.system())
        ;
    }
}

#[derive(Default)]
pub struct Mats(HashMap<&'static str, Handle<bevy::prelude::ColorMaterial>>);
impl Mats {
    pub fn get(&self, mat_name: &str) -> Handle<ColorMaterial> {
        let handle_ref = self.0.get(mat_name).expect(
            &format!("couldn't find material with name {}", mat_name));

        handle_ref.clone_weak()
    }
}

fn setup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mats: ResMut<Mats>,
) {
    let red = materials.add(Color::rgb(1., 0., 0.).into());
    mats.0.insert("red", red);

    let blue = materials.add(Color::rgb(0.5, 0.5, 1.).into());
    mats.0.insert("blue", blue);

    let gray = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    mats.0.insert("gray", gray.clone());
    mats.0.insert("grey", gray);

}
