use bevy::prelude::*;

/// Component to use for entities that are closely tied to a "main" entity.
#[derive(Component, Clone, Copy)]
pub struct Partial {
    /// Reference to the main entity this partial entity belongs to.
    main_entity: Entity,
}

impl Partial {
    pub fn new(main_entity: Entity) -> Self {
        Self { main_entity }
    }

    pub fn main_entity(&self) -> Entity {
        self.main_entity
    }
}
