use bevy::prelude::*;

use actor::{PlayerInputPlugin, MovementPlugin};

pub mod actor;
pub mod wiggle;
pub mod health;

use wiggle::WigglePlugin;
use health::HealthPlugin;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(HealthPlugin)
            .add_plugin(WigglePlugin)
            .add_plugin(PlayerInputPlugin)
            .add_plugin(MovementPlugin)
            ;
    }
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct Character {
    speed: f32
}

impl Character {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn update_speed(&mut self, change: f32) {
        self.speed += change;
    }
}
