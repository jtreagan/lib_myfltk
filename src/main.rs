
use fltk::{app, window};
use fltk::enums::Color;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

fn main() {
    let app = app::App::default();

    let mut primwin = window::Window::new(1000, 100, 700, 850, "Two Button Popup Example");
    primwin.set_color(Color::Yellow);
    primwin.end();
    primwin.show();

    app.run().unwrap();
}


