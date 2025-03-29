use fltk::{app, window::Window};
use fltk::prelude::*;


pub fn create_window() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from window.rs");
    wind.end();
    wind.show();
    app.run().unwrap();
}
