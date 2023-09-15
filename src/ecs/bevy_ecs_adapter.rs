pub type Entity = usize;

pub fn insert_entity() {}

pub fn read_entity() {}

pub fn write_entity() {}

pub fn remove_entity() {}

pub use animal_game_ecs_macros::to_bevy_component as component;
pub use animal_game_ecs_macros::to_bevy_system as system;