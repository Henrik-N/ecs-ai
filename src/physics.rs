use crate::resources_and_components::{SpriteCollider, Velocity};
use crate::util::*;
use crate::Enemy;
use bevy::{
    core::{FixedTimestep, FixedTimesteps},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    utils::{Duration, Instant},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct FixedUpdateStage;

//pub struct PhysicsPlugin;
//
//impl PhysicsPlugin {
//    pub const DEPENDENCY: &'static str = "Physics";
//}
//
//impl Plugin for PhysicsPlugin {
//    fn build(&self, app: &mut AppBuilder) {
//        app.add_system_set(SystemSet::new()
//            .label(Self::DEPENDENCY)
//            .after(crate::movement::MovementPlugin::DEPENDENCY)
//            //.with_system(check_collisions.system())
//        );
//    }
//}
//
//
//struct CollisionImpact {
//    entity: Entity,
//}

//fn check_collisions(
//    dynamic_colliders: Query<(Entity, &Transform>, With<SpriteCollider>>),
//) {
//    let mut entities = String::new();
//
//    for (entity, _transform, collider) in dynamic_colliders.iter() {
//        entities += &format!("Entity: {:?} C: {:?} | ", entity, collider);
//    }
//
//}
