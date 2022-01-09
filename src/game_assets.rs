use bevy::prelude::*;

pub struct GameAssets;
impl GameAssets {
    const DEPENDENCY: &'static str = "GameAssets";
}

impl Plugin for GameAssets {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Mats::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, setup.system());
    }
}

fn setup(mut cmd: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut mats: ResMut<Mats>) {
    let red = materials.add(Color::rgb(1., 0., 0.).into());
    mats.insert("red", red);

    let blue = materials.add(Color::rgb(0.5, 0.5, 1.).into());
    mats.insert("blue", blue);

    let gray = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    mats.insert("gray", gray.clone());
    mats.insert("grey", gray);

    let white = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    mats.insert("white", white);
}

pub use resources::*;
mod resources {
    use bevy::prelude::*;
    use derive_more::{Deref, DerefMut};
    use std::collections::HashMap;

    type MatsHashMap = HashMap<&'static str, Handle<ColorMaterial>>;

    #[derive(Default, Deref, DerefMut)]
    pub struct Mats(
        #[deref]
        #[deref_mut]
        MatsHashMap,
    );
    impl Mats {
        pub fn get(&self, mat_name: &str) -> Handle<ColorMaterial> {
            let handle_ref = self
                .0
                .get(mat_name)
                .expect(&format!("couldn't find material with name {}", mat_name));

            handle_ref.clone_weak()
        }
    }
}
