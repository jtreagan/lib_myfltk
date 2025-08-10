//! #lib_myfltk
//!
//! ## Utility functions for use with the FLTK.rs GUI crate.
//!
//! The functions in the modules below were written to help
//! with my projects that use the FLTK-RS GUI crate.
//! I've used them in several different projects
//! which is why I've kept them together in a separate crate.
//! Their greatest weakness is poor error handling, so keep that
//! in mind if you choose to use them.  By the way, I need help getting
//! those weaknesses corrected, so if you feel like taking that on,
//! please check out the issues tab in this crate's repository.
//!
//!
//!    * VERSION = "0.0.5";
//!    * AUTHOR = "John T. Reagan";
//!    * LICENSE = "MIT";
//!    * LICENSE_URL = "<https://opensource.org/licenses/MIT>";
//!    * COPYRIGHT = "Copyright (c) 2025, John T. Reagan";
//!    * REPOSITORY = "<https://github.com/jtreagan/lib_myfltk>";

/// Miscellaneous utilities for use with the FLTK-rs GUI.
/// 
pub mod fltkutils {
    use std::cell::RefCell;
    use std::mem::take;
    use std::rc::Rc;
    use fltk::{app, button, button::Button, draw, group, menu, output, text};
    use fltk::app::{quit, set_font_size, App};
    use fltk::draw::measure;
    use fltk::enums::{Color, Font, Shortcut};
    use fltk::prelude::{DisplayExt, GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt};
    use fltk::text::{TextBuffer, TextEditor};
    use fltk::window::Window;
    use lib_utils::vec::{vec_longest_str_len};
  
    /// Creates a simple, no-frills check box menu using FLTK's CheckButton struct.
    /// Returns a vector of the label strings of the boxes
    /// that were chosen by the user.
    /// Example:
    /// 
    ///     fn main() {
    ///     let app = fltk::app::App::default();
    ///   
    ///     let labels = vec!["flamingo".to_string(), "tiger".to_string(), "lion".to_string()];
    ///     
    ///     let choice = fltk_checkbox_menu(app.clone(), &labels);
    ///     
    ///     println!("\n choice: {:?} \n", choice);
    ///     
    ///     app.run().unwrap();
    ///     }
    /// 
    pub fn fltk_checkbox_menu(app: App, labels: &Vec<String>) -> Vec<String> {

        // region Setup initial window & etc.
        let mut win = Window::default().with_size(400, 300);
        let mut submit = Button::new(160, 210, 80, 40, "Submit");
        let pack = group::Pack::new(win.w()/2 - 100, 30, 100, 300, "");
        // endregion

        // region Create the checkboxes within the pack..
        let mut chkbttns = Vec::new();
        for item in labels {
            let _checkbox = button::CheckButton::default()
                .with_label(item)
                .with_size(0, 30);  // The pack takes care of positioning the checkboxes?
            chkbttns.push(_checkbox);
        }
        // endregion

        // region End the pack & window.
        pack.end();
        win.end();
        win.show();
        // endregion

        // region Do the Submit button callback.

        // Create the Rc to extract the value.  Then clone it for the callback.
        let newvec: Vec<String> = Vec::new();
        let checked = Rc::new(RefCell::new(newvec));
        let checked_clone = checked.clone();
        let chkbttns_clone = chkbttns.clone();

        submit.set_callback(move |_| {
            for item in &chkbttns_clone {
                if item.is_checked() {
                    checked_clone.borrow_mut().push(item.label().to_string());
                }
            }

            println!("\n The clone within the callback is: {:?} \n", *checked_clone.borrow_mut());
            win.hide();  // close the window

        });

        // Run the app
        while app.wait(){}

        println!("\n The checked vector after exiting the callback is: {:?} \n", *checked.borrow_mut());
        let returnvec = take(&mut *checked.borrow_mut());
        returnvec

    }

    /// Creates a checkbox shift menu of the items passed to the function
    /// in the `flist` vector.  Returns a vector of the items that were
    /// chosen by the user.
    pub fn fltk_chkbox_shift_menu(flist: &Vec<String>) -> Vec<String> {
        // todo: This uses RefCell and Rc.  Is there a better way to do it?

        let newvec: RefCell<Vec<String>> = RefCell::new(Vec::new());
        let keepers: Rc<RefCell<Vec<String>>> = Rc::new(newvec);

        let mut win = Window::default().with_size(400, 300);
        let mut row = group::Flex::default_fill().row();
        let scroll = group::Scroll::default();
        row.fixed(&scroll, 150);
        let pack = group::Pack::default().with_size(100, 300);

        for file in flist {
            let _check = button::CheckButton::default()
                .with_label(file)
                .with_size(0, 30);
        }

        pack.end();
        scroll.end();

        let mut btn = Button::default().with_label("@>");
        row.fixed(&btn, 30);
        let mut output = output::MultilineOutput::default();

        row.end();
        win.end();
        win.show();

        let keepers_clone = Rc::clone(&keepers);
        btn.set_callback(move |_b| {
            output.set_value("");
            let mut string = String::new();
            for i in 0..pack.children() {
                let check: button::CheckButton = button::CheckButton::
                from_dyn_widget(&pack.child(i).unwrap()).unwrap();
                if check.is_checked() {
                    string.push_str(&check.label());
                    string.push('\n');
                    keepers_clone.borrow_mut().push(check.label().clone());
                }
            }
            output.set_value(&string);
        });

        while win.shown() {
            app::wait();
        }

        let retvec: Vec<String> = take(&mut keepers.borrow_mut());
        retvec
    }

    /// Creates a menu of radio buttons using the `items` vector.
    /// Active items are highlighted by a small light.
    pub fn fltk_radio_lightbtn_menu(items: &Vec<String>, prompt: &str) -> String {

        // region Set up the variables.
        // todo: This uses RefCell and Rc.  Is there a better way to do it.
        let newstring: RefCell<String> = RefCell::new("".to_string());
        let keepers: Rc<RefCell<String>> = Rc::new(newstring);
        let longlen = vec_longest_str_len(items);
        // endregion

        // region Set up the window & group widgets.
        let mut win = Window::default().with_size(400, 300).with_label(prompt);
        let flex = group::Flex::default().with_size(250, 300);   // Do you really need this?
        let scroll = group::Scroll::default().with_size(200, longlen as i32 + 10);
        let pack = group::Pack::default().with_size(200, longlen as i32 + 10);  // Need this to organize the buttons.
        // endregion

        // region Create the radio buttons.
        for element in items {
            let _radio = button::RadioLightButton::default()
                .with_label(element)
                .with_size(longlen as i32, 30);
        }
        // endregion

        // region End adding widgets to the groups & create the submit button.
        pack.end();
        scroll.end();
        flex.end();

        let mut submit = Button::new(300, 175, 75, 30, "Submit");

        win.end();
        win.show();
        // endregion

        // region Use the button callbacks to get the selected radio button.
        let keepers_clone = Rc::clone(&keepers);
        let mut win_clone = win.clone();
        submit.set_callback(move |_b| {
            for i in 0..pack.children() {
                let radio: button::RadioLightButton = 
                    button::RadioLightButton::from_dyn_widget(&pack.child(i)
                        .unwrap()).unwrap();  // Complicated.  Is there a better way?
                if radio.is_toggled() {
                    *keepers_clone.borrow_mut() = radio.label().clone();
                }
            }
            win_clone.hide();
        });

        while win.shown() {
            app::wait();
        }
        // endregion

        let ret: String = keepers.borrow().clone();
        ret
    }

    /// Creates a simple, no-frills editor using FLTK's TextEditor struct.
    /// Returns the final contents of the editor.
    pub fn fltk_simple_editor(startertxt: &str, winlabel: &str) -> String {
        let edtr = App::default();
        let mut buf = TextBuffer::default();
        let mut win = Window::default().with_size(800, 300);
        set_font_size(20);
        win.set_color(Color::Yellow);
        win.set_label(winlabel);
        win.make_resizable(true);

        fltk_simple_editor_menubar();

        buf.set_text(startertxt);
        let mut simped = TextEditor::default()
            .with_size(770, 222)
            .center_of_parent();

        simped.set_buffer(buf.clone());   // Clone is used here to avoid an ownership error.
        simped.wrap_mode(text::WrapMode::AtBounds, 0);
        simped.set_color(Color::White);
        simped.set_text_size(22);
        simped.set_text_color(Color::Black);

        win.end();
        win.show();

        edtr.run().unwrap();

        buf.text()
    }

    /// Creates a menubar to be used with the `fltk_simple_editor()`.
    /// The new menubar only has ine entry -- `File/Quit` -- that provides
    /// a pattern that can be used to create more menu items
    pub fn fltk_simple_editor_menubar() -> menu::MenuBar {

        let mut menubar = menu::MenuBar::new(0, 0, 800, 40, "");

        let quit_idx = menubar.add(
            "File/Finished\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| {
                quit();
            },
        );
        menubar.at(quit_idx).unwrap().set_label_color(Color::Red);

        menubar
    }

    /// Replaces highlighted text in a `TextEditor` with the text
    /// passed in the `rpltxt` parameter.
    pub fn fltk_replace_highlighted_text(edtr: &TextEditor, buf: &mut TextBuffer, rpltxt: &str) {
        let (x, y) = match edtr.buffer().unwrap().selection_position() {
            Some(position) => position,
            None => panic!("\nError!  Could not find a cursor position in the editor.\n"),
        };

        buf.remove(x, y);                         // Remove the selected text
        buf.insert(x, rpltxt);                    // Insert new text and
        edtr.buffer().unwrap().unselect();        // Unhighlight text
    }

    /// Creates a popup window that contains two buttons.
    ///
    /// Example:
    ///
    ///     use fltk::{app, window};
    ///     use fltk::enums::Color;
    ///     use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
    ///     use lib_myfltk::fltkutils::*;
    ///
    ///     fn main() {
    ///         let app = app::App::default();
    ///
    ///         let mut primwin = window::Window::new(1000, 100, 700, 850, "Two Button Popup Example");
    ///         primwin.set_color(Color::Yellow);
    ///         primwin.end();
    ///         primwin.show();
    ///
    ///         let bttn1click = || {
    ///             println!("\n Button 1 was clicked \n");
    ///         };
    ///
    ///         let bttn2click = || {
    ///             println!("\n Button 2 was clicked \n");
    ///         };
    ///
    ///         let mut popup = fltk_popup_2btn(&primwin, Box::new(bttn1click), "Button 1",
    ///                         Box::new(bttn2click), "Button 2");
    ///
    ///         popup.end();
    ///         popup.show();
    ///
    ///         app.run().unwrap();
    ///     }
    ///
    ///
    pub fn fltk_popup_2btn(primwin: &Window, mut closure1: Box<dyn FnMut() + 'static>, label1: &str,
                           mut closure2: Box<dyn FnMut() + 'static>, label2: &str) -> Window
    {

        // region Calculate the window position -- tied to the primary window.
        let win_center = fltk_find_center_wndw(primwin);
        let popwidth = 575;  // popwidth & popheight are set to accomodate the size of the buttons.
        let popheight = 100;

        let xxx = win_center.0 - popwidth / 2;
        let yyy = win_center.1 - popheight / 2;
        // endregion

        // region Create the popup window with buttons
        let popwin = Window::default().with_size(popwidth, popheight).with_pos(xxx, yyy);

        let mut but1 = Button::new(25, 25, 250, 40, label1);
        let mut but2 = Button::new(300, 25, 250, 40, label2);
        // endregion

        // region Do the button callbacks
        let mut winclone1 = popwin.clone();
        but1.set_callback(move |_| {
            closure1();
            winclone1.hide();
        });

        let mut winclone2 = popwin.clone();
        but2.set_callback(move |_| {
            closure2();
            winclone2.hide();
        });
        // endregion

        popwin
    }

    /// Returns the coordinates of the center of `win`.
    ///
    pub fn fltk_find_center_wndw(win: &Window) -> (i32, i32) {
        let xxx = win.x();
        let yyy = win.y();
        let www = win.w();
        let hhh = win.h();

        // Calculate the center position of primwin
        let center_x = xxx + www / 2;
        let center_y = yyy + hhh / 2;

        (center_x, center_y)
    }
    
    /// Returns the dimensions of a button given a label.
    ///
    ///     #Example for  fltk_size_bttn_to_fit_label  &  fltk_center_button_in_win
    /// 
    ///      // The goal is to display a button centered in a window with both sized dynamically.
    ///
    ///     use fltk::{prelude::*, app, window::Window};
    ///     use fltk::app::{set_font, set_font_size};
    ///     use fltk::{button::Button, enums::Font};
    ///     use lib_myfltk::fltkutils::{fltk_center_button_in_win, fltk_size_bttn_to_fit_label};
    ///
    ///     const FONT: Font = Font::Helvetica;
    ///     const FONT_SIZE: i32 = 20;
    ///     const BTTN_PADDING: i32 = 30;
    ///
    ///     fn main() {
    ///         let app = app::App::default();
    ///
    ///         set_font(FONT);
    ///         set_font_size(FONT_SIZE);   // Set font & size to whatever you like best.
    ///
    ///        //let mut win1 = Window::default().with_size(40, 20);
    ///        let label = "123113333zzzzzzzzzzzffffffffffhhhhhhhhhhsdsdfdsfhj";
    ///
    ///        // Find the dimensions of the button label.
    ///        let (bttn_width, bttn_height) = fltk_size_bttn_to_fit_label(&label);
    ///
    ///        let mut win = Window::default().with_size(400, 200);
    ///
    ///        // Set the dimensions of the window to fit the button.
    ///        // (Note that the window can be any size you like as long as the button fits.)
    ///        let win_width = bttn_width + BTTN_PADDING * 2;
    ///        let win_height = bttn_height + BTTN_PADDING * 2;
    ///        win.set_size(win_width, win_height);
    ///
    ///        // Create the button.
    ///        let mut bttn = Button::default().with_size(bttn_width, bttn_height).with_label(label);
    ///
    ///        // Center the button in the window.
    ///        let (xxx, yyy) = fltk_center_button_in_win(&win, &bttn);
    ///        bttn.set_pos(xxx, yyy);
    ///
    ///        win.end();
    ///        win.show();
    ///
    ///        app.run().unwrap();
    ///     }
    ///
     pub fn fltk_size_bttn_to_fit_label(label: &str) -> (i32, i32) {

        const FONT: Font = Font::Helvetica;
        const BTTN_LABEL_PADDING: i32 = 10;

        let mut win = Window::default().with_size(40, 20);

        win.show();   // Show the window so you can measure the label.  Note that
        // the only reason you need to show the window is to measure the label.
        // The size of the window doesn't matter as far as the button is concerned.
        draw::set_font(FONT, app::font_size());  // Also needed so you can measure the label.
        let (label_len, label_height) = measure(&label, false);

        let bttn_width = label_len + BTTN_LABEL_PADDING * 2;
        let bttn_height = label_height + BTTN_LABEL_PADDING * 2;

        win.hide();
        (bttn_width, bttn_height)
    }

    /// Returns the coordinates for positioning a button centered relative to
    /// a window.  I.e. (0,0) is the top left corner of the window.  The coordinates
    /// returned by this function are for the top, left positon of the centered button.
    pub fn fltk_center_button_in_win(win: &Window, bttn: &Button) -> (i32, i32) {

        // Find coordinates to center the button both vertically and horizontally
        let (centerx, centery) = fltk_find_center_wndw(&win);
        let xxx = centerx - (bttn.w() / 2);
        let yyy = centery - (bttn.h() / 2);

        (xxx, yyy)
    }

}

/// Input functions for FLTK-RS using the `fltk::input` module.
///  
pub mod input_fltk {

/*
// todo: Still getting "unused" warnings.  Fix it if you can figure out how.

*/  // TODO's

use fltk::app::App;
use fltk::{frame, group, input, window};
use fltk::enums::{CallbackTrigger};
use fltk::prelude::{GroupExt, InputExt, WidgetExt, WindowExt};

    /// Allows the user to input a vector of Strings.
///
pub fn input_strvec(app: &App, prompt: &str, horiz: i32, vert: i32) -> Vec<String> {
    let mut list = Vec::new();
    let mut i = input_i64(app, "How many items in your list?");

    while app.wait() && i > 0 {
        let newelem = input_string(app, prompt, horiz, vert);
        list.push(newelem);
        i -= 1;
    }

    list
}

/// Allows the user to input a vector of f64 integers.
///
pub fn input_f64vec(app: &App, prompt: &str) -> Vec<f64> {
    let mut list = Vec::new();
    let mut i = input_i64(app, "How many items in your list?");



    while app.wait() && i > 0 {
        let newelem = input_f64(app, prompt);
        list.push(newelem);
        i -= 1;
    }
    list
}

/// Allows the user to input a vector of characters.
///
pub fn input_charvec(app: &App, prompt: &str) -> Vec<char> {
    let mut list = Vec::new();
    let mut i = input_i64(app, "How many items in your list?");

    while app.wait() && i > 0 {
        let newelem = input_char(app, prompt);
        list.push(newelem);
        i -= 1;
    }
    list
}

/// Allows the user to input a vector of i64 integers.
///
pub fn input_i64vec(app: &App, prompt: &str) -> Vec<i64> {
    let mut list = Vec::new();
    let mut i = input_i64(app, "How many items in your list?");

    while app.wait() && i > 0 {
        let newelem = input_i64(app, prompt);
        list.push(newelem);
        i -= 1;
    }
    list
}



/// Uses FLTK's Input widget to prompt the user to enter String data.
///
/// Works best if you set the horiz and vert to values 10 pixels less than the size
///      of the main window with the flex size set to 10 pixels less than that.
/// -- For large input windows try 790 x 490 first.
/// -- For small input windows try 300 x 90 and adjust by trial and error.
pub fn input_string(app: &App, prompt: &str, horiz: i32, vert: i32) -> String {

    // region Set up the input window and input frame
    let mut win = window::Window::default()
        .with_size(horiz, vert)
        .with_label("Input Window");
    win.make_resizable(true);

    let flex = group::Flex::default()
        .with_size(200, 75)
        .column()
        .center_of_parent();

    let _prompttext = frame::Frame::default().with_label(prompt);
    // endregion

    // region Set up the input widget inside the frame.
    let mut input_widget = input::Input::default();
    input_widget.set_trigger(CallbackTrigger::EnterKey);

    // Set the input widget's callback.
    let mut win2 = win.clone();
    input_widget.set_callback(move |_| {
        win2.hide();
    });

    flex.end();
    win.end();
    win.show();
    // endregion

    // region Deal with the input
    while win.shown() {
        app.wait();
    }
    input_widget.value()
    // endregion
}

/// Uses FLTK's Input widget to prompt the user to enter character data.
///
pub fn input_char(app: &App, prompt: &str) -> char {

    // region Set up the input window and input frame
    let mut win = window::Window::default()
        .with_size(400, 100)
        .with_label("Input Window");
    win.make_resizable(true);

    let flex = group::Flex::default()
        .with_size(200, 75)
        .column()
        .center_of_parent();

    let _prompttext = frame::Frame::default().with_label(prompt);
    // endregion

    // region Set up the input widget inside the frame.
    let mut input_widget = input::Input::default();
    input_widget.set_trigger(CallbackTrigger::EnterKey);

    // Set the input widget's callback.
    let mut win2 = win.clone();
    input_widget.set_callback(move |_| {
        win2.hide();
    });

    flex.end();
    win.end();
    win.show();
    // endregion

    // region Deal with the input
    while win.shown() {
        app.wait();
    }
    input_widget.value().chars().nth(0).unwrap()

    // endregion
}

/// Uses FLTK's Input widget to prompt the user to enter i64 integer data.
///
pub fn input_i64(app: &App, prompt: &str) -> i64 {

    // region Set up the input window and input frame
    let mut win = window::Window::default()
        .with_size(400, 100)
        .with_label("Input Window");
    win.make_resizable(true);

    let flex = group::Flex::default()
        .with_size(200, 75)
        .column()
        .center_of_parent();

    let _prompttext = frame::Frame::default().with_label(prompt);
    // endregion

    // region Set up the input widget inside the frame.
    let mut input_widget = input::IntInput::default();
    input_widget.set_trigger(CallbackTrigger::EnterKey);

    // Set the input widget's callback.
    let mut win2 = win.clone();
    input_widget.set_callback(move |_| {
        win2.hide();
    });

    flex.end();
    win.end();
    win.show();

    // endregion

    // region Deal with the input
    while win.shown() {
        app.wait();
    }
    input_widget.value().trim().parse::<i64>().unwrap()

    // endregion
}

/// Uses FLTK's Input widget to prompt the user to enter f64 data.
///
pub fn input_f64(app: &App, prompt: &str) -> f64 {

    // region Set up the input window and input frame
    let mut win = window::Window::default()
        .with_size(400, 100)
        .with_label("Input Window");
    win.make_resizable(true);

    let flex = group::Flex::default()
        .with_size(200, 75)
        .column()
        .center_of_parent();

    let _prompttext = frame::Frame::default().with_label(prompt);
    // endregion

    // region Set up the input widget inside the frame.
    let mut input_widget = input::FloatInput::default();
    input_widget.set_trigger(CallbackTrigger::EnterKey);

    // Set the input widget's callback.
    let mut win2 = win.clone();
    input_widget.set_callback(move |_| {
        win2.hide();
    });

    flex.end();
    win.end();
    win.show();

    // endregion

    // region Deal with the input
    while win.shown() {
        app.wait();
    }
    input_widget.value().trim().parse::<f64>().unwrap()

    // endregion
}


    
    
    
/*
    pub fn input_str_large(app: &App, prompt: &str, horiz: i32, vert: i32) -> String {
        // Works best if you set the horiz and vert to values 10 pixels less than the size
        //      of the main window with the flex size set to 10 pixels less than that.

        let mut win = window::Window::default()
            .with_size(horiz, vert)
            .with_label("Input Window");
        win.make_resizable(true);

        let flex = group::Flex::default()
            .with_size(horiz-10, vert-10)
            .column()
            .center_of_parent();

        let _prompttext = frame::Frame::default().with_label(prompt);

        let mut input = input::MultilineInput::default();
        input.set_wrap(true);
        input.set_trigger(CallbackTrigger::EnterKey);

        let mut win2 = win.clone();
        input.set_callback(move |_input| {
            win2.hide();
        });

        flex.end();
        win.end();
        win.show();

        while win.shown() {
            app.wait();
        }

        input.value()
    }

 */  // input_str_large()
}

/// User copyable and modifiable templates using the FLTK-RS GUI.
///   
mod templates {

/*

use fltk::button::{Button, CheckButton, RadioLightButton};
use fltk::enums::{FrameType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{FloatInput, IntInput};
use fltk::prelude::{ButtonExt, GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt};
use fltk::window::Window;

/// Creates a window simple input form that
///     utilizes radio buttons, checkboxes, frames and
///     `fltk::input` fields.
fn multivariable_input_window_template() -> Window {
    let win = Window::new(900, 100, 600, 400, "Simple Input Form");

    // region Create the radio buttons
    let radio_group = Group::new(0, 0, 600, 50, None);

    // Create horizontal radio light buttons across the top -- initial spacing.
    let bttn_w = 120;
    let bttn_h = 30;
    let spacing = 20;
    let types_xxx = 40;
    let types_yyy = 20;

    let strings_btn = RadioLightButton::new(types_xxx, types_yyy, bttn_w, bttn_h, "Strings");
    let chars_btn = RadioLightButton::new(types_xxx + bttn_w + spacing, types_yyy, bttn_w, bttn_h, "Characters");
    let mut ints_btn = RadioLightButton::new(types_xxx + 2 * (bttn_w + spacing), types_yyy, bttn_w, bttn_h, "Integers");
    let decimals_btn = RadioLightButton::new(types_xxx + 3 * (bttn_w + spacing), types_yyy, bttn_w, bttn_h, "Decimals");

    // Set Integers as default selection
    ints_btn.set_value(true);

    radio_group.end();
    // endregion

    // region Create "comma" & "list" check boxes in row below the radio buttons.

    // Calculate the position & size of the check boxes.
    let checkbox_y = types_yyy + bttn_h + 20;  // Position below radio buttons
    let checkbox_w = 150;
    let checkbox_h = 25;
    let checkbox_spacing = 30;

    let total_radio_width = bttn_w * 4 + spacing * 2;  // Width of all radio buttons + spacing
    let start_x = types_xxx + (total_radio_width - (checkbox_w * 2 + checkbox_spacing)) / 2;

    let usecommas = CheckButton::new(start_x, checkbox_y, checkbox_w, checkbox_h, "Comma Formatted");
    let fromlist = CheckButton::new(start_x + checkbox_w + checkbox_spacing, checkbox_y,
                                    checkbox_w, checkbox_h, "Value to come from a List");
    // endregion

    // region Set up frames -- for Integer & Decimal parameter entry.

    // region Set up frame parameters
    let frame_y = checkbox_y + checkbox_h + 20;  // Position below checkboxes
    let frame_w = 250;
    let input_w = 100;
    let input_h = 25;
    let label_h = 20;
    let field_spacing = 10;
    let frame_spacing = 20;
    let frame_h = 30 + (3 * (label_h + input_h + field_spacing)) + 15;
    // endregion

    // region Create Integers frame & input fields
    let mut int_frame = Group::new(types_xxx, frame_y, frame_w, frame_h, None);
    let mut int_label = Frame::new(types_xxx, frame_y, frame_w, 30, "Integer Parameters");
    int_label.set_label_size(14);

    // Calculate centered position for input fields in integer frame
    let int_input_x = types_xxx + (frame_w - input_w) / 2;
    let int_first_y = frame_y + 35; // Start below the frame label

    // Integer Minimum Value
    let _intmin_label = Frame::new(int_input_x, int_first_y, input_w, label_h, "Minimum Value");
    let intmin = IntInput::new(int_input_x, int_first_y + label_h, input_w, input_h, "");

    // Integer Maximum Value
    let _intmax_label = Frame::new(int_input_x, int_first_y + label_h + input_h + field_spacing,
                                   input_w, label_h, "Maximum Value");
    let intmax = IntInput::new(int_input_x, int_first_y + label_h + input_h + field_spacing + label_h,
                               input_w, input_h, "");

    int_frame.set_frame(FrameType::DownBox);   // Add frame border
    int_frame.end();
    // endregion

    // region Create Decimals frame & input fields
    let mut decimal_frame = Group::new(types_xxx + frame_w + frame_spacing, frame_y, frame_w, frame_h, None);
    let mut decimal_label = Frame::new(types_xxx + frame_w + frame_spacing, frame_y, frame_w, 30, "Decimal Parameters");
    decimal_label.set_label_size(14);

    // Calculate centered position for input fields in decimal frame
    let dec_input_x = types_xxx + frame_w + frame_spacing + (frame_w - input_w) / 2;
    let dec_first_y = frame_y + 35; // Start below the frame label

    // Decimal Minimum Value
    let _decmin_label = Frame::new(dec_input_x, dec_first_y, input_w, label_h, "Minimum Value");
    let decmin = FloatInput::new(dec_input_x, dec_first_y + label_h, input_w, input_h, "");

    // Decimal Maximum Value
    let _decmax_label = Frame::new(dec_input_x, dec_first_y + label_h + input_h + field_spacing,
                                   input_w, label_h, "Maximum Value");
    let decmax = FloatInput::new(dec_input_x, dec_first_y + label_h + input_h + field_spacing + label_h,
                                 input_w, input_h, "");

    // Decimal Places
    let _decplaces_label = Frame::new(dec_input_x, dec_first_y + 2 * (label_h + input_h + field_spacing),
                                      input_w, label_h, "Decimal Places");
    let decplaces = IntInput::new(dec_input_x, dec_first_y + 2 * (label_h + input_h + field_spacing) + label_h,
                                  input_w, input_h, "");

    decimal_frame.set_frame(FrameType::DownBox);   // Add frame border
    decimal_frame.end();
    // endregion

    // endregion

    // region Create the Submit button
    let submit_btn_w = 100;
    let submit_btn_h = 40;

    // Calculate center position based on the frames
    let total_frames_width = frame_w * 2 + frame_spacing;
    let submit_btn_x = types_xxx + (total_frames_width - submit_btn_w) / 2;
    let submit_btn_y = frame_y + frame_h + 20;  // 20 pixels gap after frames

    let mut submit_btn = Button::new(submit_btn_x, submit_btn_y, submit_btn_w, submit_btn_h, "Submit");
    // endregion

    // region Set up the callback for the Submit button

    // Clone buttons and window for the callback
    let strings_btn = strings_btn.clone();
    let chars_btn = chars_btn.clone();
    let ints_btn = ints_btn.clone();
    let decimals_btn = decimals_btn.clone();
    let mut win_clone = win.clone();

    submit_btn.set_callback(move |_| {

        // region Deal with the radio buttons.
        let vrbl_type = if strings_btn.value() {
            "Strings"
        } else if chars_btn.value() {
            "Characters"
        } else if ints_btn.value() {
            "Integers"
        } else if decimals_btn.value() {
            "Decimals"
        } else {
            "None"
        };

        println!("\n Selected Type: {} \n", vrbl_type);
        // endregion

        //region Deal with the "comma" & "list" check boxes.
        if usecommas.is_checked() {
            print!("\n Comma Formatted == true \n");
        }  else {
            print!("\n Comma Formatted == false \n");
        }

        if fromlist.is_checked() {
            print!("\n List == true \n");
        }  else {
            print!("\nList == false \n");
        }
        // endregion

        // region Deal with the Integer input fields.
        if vrbl_type == "Integers" {
            println!("\n Integer Minimum Value: {}", intmin.value());
            println!(" Integer Maximum Value: {} \n", intmax.value());
        }
        // endregion

        // region Deal with the Decimal input fields.
        if vrbl_type == "Decimals" {
            println!("\n Decimal Minimum Value: {}", decmin.value());
            println!(" Decimal Maximum Value: {}", decmax.value());
            println!(" Decimal Places: {} \n", decplaces.value());
        }
        // endregion

        // Close the window
        win_clone.hide();
    });
    // endregion

    win.end();
    win
}

/// Here's what it looks like in the Question Bank application.
///
pub fn vrbl_parameters_input_box(var1: &mut Variable) {

    // todo: Grey out the input fields when the variable type is not "int" or "float".
    //          Use the `deactivate()` method.  First attempt didn't work.

    let mut win = Window::new(900, 100, 600, 400, "Variable Parameters");
    win.set_color(Color::Cyan);
    win.make_resizable(true);

    // region Create the radio buttons for the variable type.
    let radio_group = Group::new(0, 0, 600, 50, None);

    // Create horizontal radio light buttons across the top -- initial spacing.
    let bttn_w = 120;
    let bttn_h = 30;
    let spacing = 20;
    let types_xxx = 40;
    let types_yyy = 20;

    let mut strings_btn = RadioLightButton::new(types_xxx, types_yyy, bttn_w, bttn_h, "Strings");
    let chars_btn = RadioLightButton::new(types_xxx + bttn_w + spacing, types_yyy, bttn_w, bttn_h, "Characters");
    let ints_btn = RadioLightButton::new(types_xxx + 2 * (bttn_w + spacing), types_yyy, bttn_w, bttn_h, "Integers");
    let decimals_btn = RadioLightButton::new(types_xxx + 3 * (bttn_w + spacing), types_yyy, bttn_w, bttn_h, "Decimals");

    // Set Integers as default selection
    strings_btn.set_value(true);

    radio_group.end();
    // endregion

    // region Create "comma" & "list" check boxes in row below the radio buttons.

    // Calculate the position & size of the check boxes.
    let ckbx_y = types_yyy + bttn_h + 20;  // Position below radio buttons
    let ckbx_w = 150;
    let ckbx_h = 25;
    let ckbx_spacing = 55;

    let total_radio_width = bttn_w * 4 + spacing * 2;  // Width of all radio buttons + spacing
    let start_x = types_xxx + (total_radio_width - (ckbx_w * 2 + ckbx_spacing)) / 2;

    // Create the check boxes.
    let usecommas = CheckButton::new(start_x, ckbx_y, ckbx_w, ckbx_h, "Comma Formatted");
    let fromlist = CheckButton::new(start_x + ckbx_w + ckbx_spacing, ckbx_y,
                                    ckbx_w, ckbx_h, "Value to come from a List");


    // endregion

    // region Set up frames -- for Integer & Decimal parameter entry.

    // region Set up frame parameters
    let frame_y = ckbx_y + ckbx_h + 20;  // Position below checkboxes
    let frame_w = 250;
    let input_w = 100;
    let input_h = 25;
    let label_h = 20;
    let field_spacing = 10;
    let frame_spacing = 20;
    let frame_h = 30 + (3 * (label_h + input_h + field_spacing)) + 15;
    // endregion

    // region Create Integers frame & input fields
    let mut int_frame = Group::new(types_xxx, frame_y, frame_w, frame_h, None);
    let mut int_label = Frame::new(types_xxx, frame_y, frame_w, 30, "Integer Parameters");
    int_label.set_label_size(14);

    // Calculate centered position for input fields in integer frame
    let int_input_x = types_xxx + (frame_w - input_w) / 2;
    let int_first_y = frame_y + 35; // Start below the frame label

    // Integer Minimum Value
    let _intmin_label = Frame::new(int_input_x, int_first_y, input_w, label_h, "Minimum Value");
    let mut intmin = IntInput::new(int_input_x, int_first_y + label_h, input_w, input_h, "");

    // Integer Maximum Value
    let _intmax_label = Frame::new(int_input_x, int_first_y + label_h + input_h + field_spacing,
                                   input_w, label_h, "Maximum Value");
    let mut intmax = IntInput::new(int_input_x, int_first_y + label_h + input_h + field_spacing + label_h,
                                   input_w, input_h, "");

    int_frame.set_frame(FrameType::DownBox);   // Add frame border
    int_frame.end();
    // endregion

    // region Create Decimals frame & input fields
    let mut decimal_frame = Group::new(types_xxx + frame_w + frame_spacing, frame_y, frame_w, frame_h, None);
    let mut decimal_label = Frame::new(types_xxx + frame_w + frame_spacing, frame_y, frame_w, 30, "Decimal Parameters");
    decimal_label.set_label_size(14);

    // Calculate centered position for input fields in decimal frame
    let dec_input_x = types_xxx + frame_w + frame_spacing + (frame_w - input_w) / 2;
    let dec_first_y = frame_y + 35; // Start below the frame label

    // Decimal Minimum Value
    let _decmin_label = Frame::new(dec_input_x, dec_first_y, input_w, label_h, "Minimum Value");
    let mut decmin = FloatInput::new(dec_input_x, dec_first_y + label_h, input_w, input_h, "");

    // Decimal Maximum Value
    let _decmax_label = Frame::new(dec_input_x, dec_first_y + label_h + input_h + field_spacing,
                                   input_w, label_h, "Maximum Value");
    let mut decmax = FloatInput::new(dec_input_x, dec_first_y + label_h + input_h + field_spacing + label_h,
                                     input_w, input_h, "");

    // Decimal Places
    let _decplaces_label = Frame::new(dec_input_x, dec_first_y + 2 * (label_h + input_h + field_spacing),
                                      input_w, label_h, "Decimal Places");
    let mut decplaces = IntInput::new(dec_input_x, dec_first_y + 2 * (label_h + input_h + field_spacing) + label_h,
                                      input_w, input_h, "");

    decimal_frame.set_frame(FrameType::DownBox);   // Add frame border
    decimal_frame.end();
    // endregion

    // endregion

    // region Create the Submit button
    let submit_btn_w = 100;
    let submit_btn_h = 40;

    // Calculate center position based on the frames
    let total_frames_width = frame_w * 2 + frame_spacing;
    let submit_btn_x = types_xxx + (total_frames_width - submit_btn_w) / 2;
    let submit_btn_y = frame_y + frame_h + 20;  // 20 pixels gap after frames

    let mut submit_btn = Button::new(submit_btn_x, submit_btn_y, submit_btn_w, submit_btn_h, "Submit");
    // endregion

    win.end();
    win.show();

    // region Clone variables for the callback
    let strings_btn = strings_btn.clone();
    let chars_btn = chars_btn.clone();
    let ints_btn = ints_btn.clone();
    let decimals_btn = decimals_btn.clone();
    let mut win_clone = win.clone();

    let datavar = Rc::new(RefCell::new(Variable::new()));
    let datavar_outside = datavar.clone();  // Create a second Rc pointing to the same RefCell


    // endregion

    // region Do the callback for the Submit button
    submit_btn.set_callback(move |_| {

        // region Deal with the radio buttons.
        let vartype = if strings_btn.value() {
            decmin.deactivate();
            decmax.deactivate();
            decplaces.deactivate();
            intmin.deactivate();
            intmax.deactivate();
            "Strings"
        } else if chars_btn.value() {
            decmin.deactivate();
            decmax.deactivate();
            decplaces.deactivate();
            intmin.deactivate();
            intmax.deactivate();
            "Characters"
        } else if ints_btn.value() {
            decmin.deactivate();
            decmax.deactivate();
            decplaces.deactivate();
            "Integers"
        } else if decimals_btn.value() {
            intmin.deactivate();
            intmax.deactivate();
            "Decimals"
        } else {
            "None"
        };

        datavar.borrow_mut().var_type = vartype.to_string();
        // endregion

        //region Deal with the "comma" & "list" check boxes.
        if usecommas.is_checked() {
            datavar.borrow_mut().params.num_comma_frmttd = true;
            print!("\n Comma Formatted == true \n");
        }  else {
            datavar.borrow_mut().params.num_comma_frmttd = false;
            print!("\n Comma Formatted == false \n");
        }

        if fromlist.is_checked() {
            datavar.borrow_mut().params.is_from_list = true;
            print!("\n List == true \n");
        }  else {
            datavar.borrow_mut().params.is_from_list = false;
            print!("\nList == false \n");
        }
        // endregion

        // region Deal with the Integer input fields.
        if vartype == "Integers" {
            datavar.borrow_mut().params.is_int = true;
            datavar.borrow_mut().params.is_float = false;
            datavar.borrow_mut().params.num_min_int = intmin.value().parse::<i64>().unwrap();
            datavar.borrow_mut().params.num_max_int = intmax.value().parse::<i64>().unwrap();
        }
        // endregion

        // region Deal with the Decimal input fields.
        if vartype == "Decimals" {
            datavar.borrow_mut().params.is_int = false;
            datavar.borrow_mut().params.is_float = true;
            datavar.borrow_mut().params.num_min_float = decmin.value().parse::<f64>().unwrap();
            datavar.borrow_mut().params.num_max_float = decmax.value().parse::<f64>().unwrap();
            datavar.borrow_mut().params.num_dcml_places = decplaces.value().parse::<usize>().unwrap();
        }
        // endregion

        println!("\n In the callback, datavar == {:?} \n", datavar);

        // Close the window
        win_clone.hide();
    });
    // endregion

    // Keep window active until hidden
    while win.shown() {
        app::wait();
    }

    *var1 = datavar_outside.borrow().clone();
}

*/   // Unused code below.

}