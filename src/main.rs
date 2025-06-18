
// The goal is to display a button centered in a window with both sized dynamically.

use fltk::{prelude::*, app, window::Window};
use fltk::app::{set_font, set_font_size};
use fltk::{button::Button, enums::Font};
use lib_myfltk::fltkutils::{fltk_center_button_in_win, fltk_size_bttn_to_fit_label};

const FONT: Font = Font::Helvetica;
const FONT_SIZE: i32 = 20;
const BTTN_PADDING: i32 = 30;

fn main() {
    let app = app::App::default();

    set_font(FONT);
    set_font_size(FONT_SIZE);   // Set font & size to whatever you like best.

    //let mut win1 = Window::default().with_size(40, 20);
    let label = "123113333zzzzzzzzzzzffffffffffhhhhhhhhhhsdsdfdsfhj";

    // Find the dimensions of the button label.
    let (bttn_width, bttn_height) = fltk_size_bttn_to_fit_label(&label);

    let mut win = Window::default().with_size(400, 200);

    // Set the dimensions of the window to fit the button.
    // (Note that the window can be any size you like as long as the button fits.)
    let win_width = bttn_width + BTTN_PADDING * 2;
    let win_height = bttn_height + BTTN_PADDING * 2;
    win.set_size(win_width, win_height);
        
    // Create the button.
    let mut bttn = Button::default().with_size(bttn_width, bttn_height).with_label(label);

    // Center the button in the window.
    let (xxx, yyy) = fltk_center_button_in_win(&win, &bttn);
    bttn.set_pos(xxx, yyy);

    win.end();
    win.show();

    app.run().unwrap();
}




