use bevy_app::prelude::*;
use bevy_app::CreatePlugin;
use libloading::{Library, Symbol};
fn main() {
    let mut app = App::new();
    unsafe {
        app.load_plugin("src/lib/src/lib.rs");
    }
    app.run();
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
