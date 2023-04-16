use bevy::prelude::*;


pub struct AiEventsPlugin;

pub struct EventDistanceReached {
    pub parent: Entity,
}

pub struct EventDistanceExited {
    pub parent: Entity,
}


impl Plugin for AiEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventDistanceReached>();
        app.add_event::<EventDistanceExited>();
    }
}