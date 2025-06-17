
use fltk::{prelude::*, *, app, window::Window};
use fltk::app::{set_font, set_font_size};
use fltk::{button::Button, draw::measure, enums::Font};

// The goal is to display a button centered in a window with both sized dynamically.

const FONT: Font = Font::Helvetica;
const FONT_SIZE: i32 = 20;
const LABEL_PADDING: i32 = 10;
const BTN_PADDING: i32 = 0;

fn main() {
    let app = app::App::default();

    set_font(FONT);
    set_font_size(FONT_SIZE);   // Set font & size to whatever you like best.

    let mut win = Window::default().with_size(300, 100);
    let label = "123113333sdsdfdsfhj";

    // Find the dimensions of the button label.
    let (bttn_width, bttn_height) = fltk_size_bttn_to_fit_label(&mut win, &label);


    // Set the dimensions of the window to fit the button.
    // (Note that the window can be any size you like as long as the button fits.)
    let win_width = bttn_width + BTN_PADDING * 2;
    let win_height = bttn_height + BTN_PADDING * 2;
    win.set_size(win_width, win_height);
    win.begin();  // I'm not sure just why this is needed, but it is.

    // Create the button.
    let mut bttn = Button::default().with_size(bttn_width, bttn_height).with_label(label);


    // Center the button in the window.
    let (xxx, yyy) = fltk_center_button_in_win(&win, &bttn);
    bttn.set_pos(xxx, yyy);


    win.end();
    win.show();

    app.run().unwrap();
}


/// Returns the dimensions of a button given a label.
///
pub fn fltk_size_bttn_to_fit_label(win: &mut Window, label: &str) -> (i32, i32) {
    win.show();   // Show the window so you can measure the label.
    draw::set_font(FONT, app::font_size());  // Also needed to measure the label.
    let (label_len, label_height) = measure(&label, false);

    let bttn_width = label_len + LABEL_PADDING * 2;
    let bttn_height = label_height + LABEL_PADDING * 2;

    win.hide();
    (bttn_width, bttn_height)
}

/// Return the coordinates for positioning a button centered relative to
/// a window.  I.e. (0,0) is the top left corner of the window.  The coordinates
/// returned by this function are for the top, left positon of the centered button.
pub fn fltk_center_button_in_win(win: &Window, bttn: &Button) -> (i32, i32) {

    // Find coordinates to enter the button both vertically and horizontally
    let (centerx, centery) = fltk_center_of_win(&win);
    let xxx = centerx - (bttn.w() / 2);
    let yyy = centery - (bttn.h() / 2);

    (xxx, yyy)
}

/// Return the coordinates of the center point of a window relative to
/// the window.  I.e. (0,0) is the top left corner of the window.
pub fn fltk_center_of_win(win: &Window) -> (i32, i32) {
    let winwidth = win.w();
    let winheight = win.h();
    let wincenterx = winwidth / 2;
    let wincentery = winheight / 2;

    (wincenterx, wincentery)
}