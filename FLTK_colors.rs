use fltk::app::App;
use fltk::enums::{Color, FrameType};
use fltk::frame::Frame;
use fltk::group::Scroll;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::Window;

fn main() {
    let app = App::default();
    let mut win = Window::new(900, 100, 400, 700, "FLTK Background Colors with Scroll");

    // Create the Scroll widget
    let mut scroll = Scroll::new(0, 0, 400, 700, None);

    // Frame position and spacing settings
    let mut frame_y = 25; // Start y position
    let frame_height = 100; // Height of each frame, plus spacing between frames

    // Create Frames for all Color constants inside the Scroll widget
    let colors = [
        (Color::White, "Color::White"),
        (Color::Black, "Color::Black"),
        (Color::Red, "Color::Red"),
        (Color::Green, "Color::Green"),
        (Color::Blue, "Color::Blue"),
        (Color::Yellow, "Color::Yellow"),
        (Color::Cyan, "Color::Cyan"),
        (Color::Magenta, "Color::Magenta"),
        (Color::DarkRed, "Color::DarkRed"),
        (Color::DarkGreen, "Color::DarkGreen"),
        (Color::DarkBlue, "Color::DarkBlue"),
        (Color::DarkYellow, "Color::DarkYellow"),
        (Color::DarkCyan, "Color::DarkCyan"),
        (Color::DarkMagenta, "Color::DarkMagenta"),
        (Color::Dark1, "Color::Dark1"),
        (Color::Dark2, "Color::Dark2"),
        (Color::Dark3, "Color::Dark3"),
        (Color::Light1, "Color::Light1"),
        (Color::Light2, "Color::Light2"),
        (Color::Light3, "Color::Light3"),
    ];

    // Loop to dynamically create frames for all colors
    for (color, label) in colors.iter() {
        let mut frame = Frame::new(25, frame_y, 200, 75, Some(*label));
        frame.set_frame(FrameType::RShadowBox);
        frame.set_color(*color);
        frame_y += frame_height; // Increment y-position for the next frame
    }

    // Add a dummy widget to extend the scroll area
    Frame::new(0, frame_y, 0, 0, ""); // This won't be visible, but it helps set the scroll bounds

    scroll.end(); // Finalize the scroll widget

    win.end(); // Finalize the window
    win.show();

    app.run().unwrap();
}