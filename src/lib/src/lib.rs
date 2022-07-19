use bevy::prelude::*;

#[derive(DynamicPlugin)]
pub struct MyApp;
impl Plugin for MyApp {
    fn build(&self, app: &mut App) {
        //app.add_default_stages();
        for stage in app.schedule.iter_stages() {
            println!("{:?}", stage.0);
        }
        app.add_system(system);
    }
}

fn system() {
    println!("hello from DynamicApp!");
}
