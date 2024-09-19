#![allow(dead_code)]
#![allow(unused)]

/*
                Goals

    -- Experiment with imputting text
        and retrieving that text.
    -- Create a widget to input a file name.
    -- Create a widget with checkboxes
        to allow for choosing parameters.

*/

use fltk::enums::{Color, Font, FrameType};
use fltk::{app, button::Button, dialog, enums, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();

    app::set_font(enums::Font::Times);
    app::set_font_size(22);

    //let mut wind = Window::new(500, 200, 400, 300, "JT's Window!");
    // let mut frame = Frame::new(100, 50, 200, 100, "Frame Label");

    //frame.set_frame(FrameType::GleamDownBox);
    //frame.set_color(Color::Cyan); // Frame background color.

    let filename = dialog::input(900, 50, "Please enter a file name", "").unwrap();
    //wind.end();
    //wind.show();

    app.run().unwrap();

    println!("/n The file name is:  {}/n", filename);
}
