#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::any::Any;

pub trait Plugin : Sync + Send {
    fn init(&mut self, args: &HashMap<String, Box<Any>>);
    fn spawn(&self);
}

pub struct PluginRegistry {
    plugins: HashMap<String, Box<Plugin>>
}

pub trait PluginRegister {
    fn register();
}

impl PluginRegistry {
    fn new() -> PluginRegistry {
        PluginRegistry {
            plugins: HashMap::new()
        }
    }

    pub fn register(&mut self, name: String, plugin: Box<Plugin>) {
        self.plugins.insert(name, plugin);
    }

    pub fn query_mut(&mut self, name: &str) -> Option<&mut Box<Plugin>> {
        self.plugins.get_mut(name)
            .and_then(|boxed| Some(boxed))
    }
}

use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    pub static ref PLUGIN_REGISTRY: Arc<Mutex<PluginRegistry>> 
        = Arc::new(Mutex::new(PluginRegistry::new()));
}

