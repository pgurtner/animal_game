use bevy_ecs::bundle::Bundle;
use bevy_ecs::world::{EntityMut, EntityRef};

pub use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::IntoSystemConfigs;

pub struct ECSWorld {
    world: bevy_ecs::world::World
}

impl ECSWorld {
    pub fn new () -> Self {
        ECSWorld {
            world: bevy_ecs::world::World::default()
        }
    }

    pub fn create_entity<B: Bundle>(&mut self, bundle: B) -> EntityMut {
        self.world.spawn(bundle)
    }

    pub fn get_entity (&self, entity: Entity) -> Option<EntityRef> {
        self.world.get_entity(entity)
    }

    pub fn get_entity_mut (&mut self, entity: Entity) -> Option<EntityMut> {
        self.world.get_entity_mut(entity)
    }

    pub fn remove_entity (&mut self, entity: Entity) -> bool {
        self.world.despawn(entity)
    }
}

pub struct ECSSchedule {
    schedule: bevy_ecs::schedule::Schedule
}

impl ECSSchedule {
    pub fn new () -> Self {
        ECSSchedule {
            schedule: bevy_ecs::schedule::Schedule::default()
        }
    }

    pub fn add_system<M> (&mut self, system: impl IntoSystemConfigs<M>) {
        self.schedule.add_systems(system);
    }

    pub fn run (&mut self, world: &mut ECSWorld) {
        self.schedule.run(&mut world.world);
    }
}

pub use animal_game_ecs_macros::to_bevy_component as component;
pub use animal_game_ecs_macros::to_bevy_system as system;