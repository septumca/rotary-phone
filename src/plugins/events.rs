use bevy::prelude::*;

use crate::components::EquippedSkill;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_event::<SkillEvent>();
    }
}

pub struct SkillEvent {
    pub kind: EquippedSkill,
    pub parent: Entity,
    pub angle: f32,
    pub start_position: Vec2,
    pub spawn_vector_norm: Vec2,
}
