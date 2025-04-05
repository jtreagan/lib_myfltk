
/*                            Goals

    -- Done!!!

*/

use fltk::{app, window};
use fltk::enums::Color;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use lib_myfltk::fltkutils::*;

fn main() {
    let app = app::App::default();

    let mut primwin = window::Window::new(1000, 100, 700, 850, "Two Button Popup Example");
    primwin.set_color(Color::Yellow);
    primwin.end();
    primwin.show();

    let bttn1click = || {
        println!("\n Button 1 was clicked \n");
    };

    let bttn2click = || {
        println!("\n Button 2 was clicked \n");
    };

    let mut popup = fltk_popup_2btn(&primwin, Box::new(bttn1click), "Button 1",
                    Box::new(bttn2click), "Button 2");

    popup.end();
    popup.show();

    app.run().unwrap();
}


