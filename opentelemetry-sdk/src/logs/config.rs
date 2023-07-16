use std::borrow::Cow;

use crate::logs::EventEnabled;
use crate::Resource;

use super::DefaultEventEnabled;

/// Log emitter configuration.
#[derive(Debug)]
pub struct Config {
    /// Contains attributes representing an entity that produces telemetry.
    pub resource: Cow<'static, Resource>,
    /// The event enabled implementation to use.
    pub event_enabled: Box<dyn EventEnabled>,
}

impl Config {
    /// Specify the attributes representing the entity that produces telemetry
    pub fn with_resource(mut self, resource: Resource) -> Self {
        self.resource = Cow::Owned(resource);
        self
    }
}

impl Default for Config {
    /// Create default global sdk configuration.
    fn default() -> Self {
        Config {
            event_enabled: Box::new(DefaultEventEnabled::default()),
            resource: Cow::Owned(Resource::default()),
        }
    }
}
