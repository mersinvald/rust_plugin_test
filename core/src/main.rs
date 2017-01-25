extern crate plugin_api;
extern crate plugin_pow;

use std::collections::HashMap;
use std::any::Any;
use plugin_api::Plugin;
use plugin_api::PluginRegister;

fn main() {
    plugin_pow::Register::register();

    let arc_registry = plugin_api::PLUGIN_REGISTRY.clone();
    let mut registry =  arc_registry.lock().unwrap();

    let pow_plugin = registry.query_mut("pow").unwrap();

    let mut pow_args = HashMap::new();
    pow_args.insert(String::from("x"), Box::new(3) as Box<Any>);
    pow_args.insert(String::from("pow"), Box::new(3) as Box<Any>);

    pow_plugin.init(&pow_args);
    pow_plugin.spawn();

    pow_args.insert(String::from("x"), Box::new(15) as Box<Any>);
    pow_args.insert(String::from("pow"), Box::new(3) as Box<Any>);

    pow_plugin.init(&pow_args);
    pow_plugin.spawn();

    std::thread::sleep_ms(10000);
}