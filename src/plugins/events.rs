use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_event::<FireEvent>()
          .add_event::<SlashEvent>();
    }
}

pub struct SlashEvent {
    pub angle: f32,
    pub start_position: Vec2,
    pub spawn_vector_norm: Vec2,
}
pub struct FireEvent {
    pub angle: f32,
    pub start_position: Vec2,
    pub spawn_vector_norm: Vec2,
}
