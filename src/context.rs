use crate::resource::ResourceRegistry;

pub struct Context {
    pub resources: ResourceRegistry,
}

impl Context {
    pub fn new() -> Self {
        Self {
            resources: ResourceRegistry::new(),
        }
    }

    pub fn register_resource<T: Send + Sync + 'static>(&mut self, data: T) {
        self.resources.insert(data);
    }

    pub fn get_resource<T: Send + Sync + 'static>(&self) -> Option<std::sync::Arc<T>> {
        self.resources.get::<T>()
    }
}
