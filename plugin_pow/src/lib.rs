#[macro_use]
extern crate lazy_static;
extern crate plugin_api;

use std::collections::HashMap;
use std::any::Any;
use std::thread;

#[derive(Default, Copy, Clone, Debug)]
struct PluginPow {
    x: i32,
    pow: i32,
}

impl plugin_api::Plugin for PluginPow {
    fn init(&mut self, mut args: &HashMap<String, Box<Any>>) {
        let x_arg = args.get("x").expect("Expected i32 argument 'x'");
        let pow_arg = args.get("pow").expect("Expected i32 argument 'pow'");

        self.x = *(*x_arg).downcast_ref::<i32>().unwrap();
        self.pow = *(*pow_arg).downcast_ref::<i32>().unwrap();
    }

    fn spawn(&self) {
        let pause = std::time::Duration::from_secs(1);
        let copy = self.clone();
        thread::spawn(move || {
            loop {
                let pow = copy.x.pow(copy.pow as u32);
                println!("{}", pow);
                thread::sleep(pause);
            }
        });
    }
}

use plugin_api::PluginRegister;

pub struct Register;
impl PluginRegister for Register {
    fn register() {
        let arc_reg = plugin_api::PLUGIN_REGISTRY.clone();
        let mut reg = arc_reg.lock().unwrap();
        reg.register(
            String::from("pow"), Box::new(PluginPow::default())
        );
    }
}
