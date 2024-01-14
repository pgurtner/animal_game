mod bevy_ecs_adapter;

pub use bevy_ecs_adapter::{ECSWorld, ECSSchedule, ECSEntity, component, system};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use rand::prelude::*;

    #[component]
    #[derive(Debug, Clone)]
    struct Position {x: u64, y: u64}

    #[component]
    #[derive(Clone)]
    struct Velocity {x: u64, y: u64}

    #[system]
    fn movement (pos: &mut Position, v: &Velocity, entity: ECSEntity) {
        let _ = entity;
        pos.x += v.x;
        pos.y += v.y;
    }

    #[test]
    fn test_simple_calculation_system () {
        let mut world = ECSWorld::new();
        let mut entity_storage = HashMap::new();
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let entity_comps = (
                Position{x: rng.next_u32() as u64, y: rng.next_u32() as u64},
                Velocity{x: rng.next_u32() as u64, y: rng.next_u32() as u64}
            );

            entity_storage.insert(
                world.create_entity(entity_comps.clone()).id(),
                entity_comps
            );
        }

        let mut schedule = ECSSchedule::new();

        schedule.add_system(movement);

        schedule.run(&mut world);

        entity_storage
            .iter()
            .map(|(entity_id, old_entity)| (world.get_entity(*entity_id).unwrap(), old_entity))
            .for_each(|(new_entity, old_entity)| {
                let pos = new_entity.get::<Position>().unwrap();

                assert_eq!(pos.x, old_entity.0.x + old_entity.1.x);
                assert_eq!(pos.y, old_entity.0.y + old_entity.1.y);
            });
    }
}