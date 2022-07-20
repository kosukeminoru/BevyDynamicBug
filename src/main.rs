use bevy::app::CreatePlugin;
use bevy::prelude::*;
use libloading::{Library, Symbol};
use std::time::Duration;
fn main() {
    let mut app = App::new();
    app.add_plugins(bevy::MinimalPlugins);

    app.add_system(system2);
    app.insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
        Duration::from_secs(0),
    ));
    unsafe {
        app.load_plugin("src/lib/target/debug/libbevylib.dylib");
    }
    app.run();
}
fn system2() {
    println!("hello from main app!");
}

pub unsafe fn dynamically_load_plugin(path: &str) -> (Library, Box<dyn Plugin>) {
    let lib = Library::new(path).unwrap();
    let func: Symbol<CreatePlugin> = lib.get(b"_bevy_create_plugin").unwrap();
    let plugin = Box::from_raw(func());
    (lib, plugin)
}

pub trait DynamicPluginExt {
    /// # Safety
    ///
    /// Same as [`dynamically_load_plugin`].
    unsafe fn load_plugin(&mut self, path: &str) -> &mut Self;
}

impl DynamicPluginExt for App {
    unsafe fn load_plugin(&mut self, path: &str) -> &mut Self {
        let (lib, plugin) = dynamically_load_plugin(path);
        std::mem::forget(lib); // Ensure that the library is not automatically unloaded
        plugin.build(self);
        self
    }
}
