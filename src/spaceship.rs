use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::app::{App, Plugin, Startup};
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        acceleration: Acceleration::new(Vec3::ZERO),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        model: SceneRoot(scene_assets.spaceship.clone()),
    });
}
