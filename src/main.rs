 #![windows_subsystem = "windows"]
use hello_world::App;

fn main() {
    let mut app = App::App::new().unwrap();
    app.run().unwrap();
}
