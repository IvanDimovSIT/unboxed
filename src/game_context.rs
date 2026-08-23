use crate::{
    level::{Level, LevelContext},
    service::movement::MovementDelta,
};

#[derive(Debug)]
pub struct GameContext<'a> {
    pub level_templates: &'a [Level],
    pub level_context: LevelContext,
    pub animation_time_s: f32,
    pub animation_deltas: Vec<MovementDelta>,
}
