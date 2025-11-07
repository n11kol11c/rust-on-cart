use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

pub struct ResourceRegistry {
    map: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert<T: Send + Sync + 'static>(&self, value: T) {
        self.map.write().insert(TypeId::of::<T>(), Arc::new(value));
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.map
            .read()
            .get(&TypeId::of::<T>())
            .and_then(|value| value.clone().downcast::<T>().ok())
    }
}
