use hecs::{Entity, World as HWorld};

/// The simulation world — wraps `hecs::World` and provides the ECS storage.
pub struct World {
    inner: HWorld,
}

impl World {
    pub fn new() -> Self {
        Self {
            inner: HWorld::new(),
        }
    }

    /// Spawn a new entity with the given component bundle. Returns its ID.
    pub fn spawn(&mut self, components: impl hecs::DynamicBundle) -> Entity {
        self.inner.spawn(components)
    }

    /// Despawn an entity, removing it and all its components.
    pub fn despawn(&mut self, entity: Entity) -> Result<(), hecs::NoSuchEntity> {
        self.inner.despawn(entity)
    }

    /// Get a reference to a component on an entity.
    pub fn get<T: Send + Sync + 'static>(
        &self,
        entity: Entity,
    ) -> Result<hecs::Ref<'_, T>, hecs::ComponentError> {
        self.inner.get::<&T>(entity)
    }

    /// Get a mutable reference to a component on an entity.
    pub fn get_mut<T: Send + Sync + 'static>(
        &self,
        entity: Entity,
    ) -> Result<hecs::RefMut<'_, T>, hecs::ComponentError> {
        self.inner.get::<&mut T>(entity)
    }

    /// Query for entities matching a component set.
    pub fn query<Q: hecs::Query>(&self) -> hecs::QueryBorrow<'_, Q> {
        self.inner.query::<Q>()
    }

    /// Query for entities matching a component set (mutable).
    pub fn query_mut<Q: hecs::Query>(&mut self) -> hecs::QueryMut<'_, Q> {
        self.inner.query_mut::<Q>()
    }

    /// Insert a component into an entity (adding it if not present, replacing if present).
    pub fn insert_one<T: Send + Sync + 'static>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<(), hecs::NoSuchEntity> {
        self.inner.insert_one(entity, component)
    }

    /// Remove a component from an entity.
    pub fn remove_one<T: Send + Sync + 'static>(
        &mut self,
        entity: Entity,
    ) -> Result<T, hecs::ComponentError> {
        self.inner.remove_one::<T>(entity)
    }

    /// Returns the number of entities currently in the world.
    pub fn len(&self) -> u32 {
        self.inner.len()
    }

    /// Returns true if the world contains no entities.
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    /// Provides direct access to the underlying hecs World for advanced use.
    pub fn inner(&self) -> &HWorld {
        &self.inner
    }

    /// Provides mutable access to the underlying hecs World for advanced use.
    pub fn inner_mut(&mut self) -> &mut HWorld {
        &mut self.inner
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
