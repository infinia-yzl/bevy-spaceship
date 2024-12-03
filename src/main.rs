use bevy::prelude::*;
// use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use camera::CameraPlugin;
use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroids::AsteroidPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::despawn::DespawnPlugin;

mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
