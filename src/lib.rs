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


pub mod fltkutils {
    use std::cell::RefCell;
    use std::mem::take;
    use std::rc::Rc;
    use fltk::{app, button, button::Button, group, menu, output, text};
    use fltk::app::{quit, set_font_size, App};
    use fltk::enums::{Color, Shortcut};
    use fltk::prelude::{DisplayExt, GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt};
    use fltk::text::{TextBuffer, TextEditor};
    use fltk::window::Window;
    use lib_utils::vec::vec_longest_str_len;

    /// Creates a checkbox shift menu of the items passed to the function
    /// in the `flist` vector.  Returns a vector of the items that were
    /// chosen by the user.
    pub fn fltk_chkbox_shift_menu(flist: &Vec<String>) -> Vec<String> {
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
    pub fn fltk_radio_lightbtn_menu(items: &Vec<String>) -> String {

        // region Set up the variables.
        // todo: This uses RefCell and Rc.  Is there a better way to do it.
        let newstring: RefCell<String> = RefCell::new("".to_string());
        let keepers: Rc<RefCell<String>> = Rc::new(newstring);
        let longest = vec_longest_str_len(&items);
        // endregion

        // region Set up the window & group widgets.
        let mut win = Window::default().with_size(400, 300);
        let flex = group::Flex::default().with_size(250, 300);   // Do you really need this?
        let scroll = group::Scroll::default().with_size(200, longest as i32 + 10);
        let pack = group::Pack::default().with_size(200, longest as i32 + 10);  // Need this to organize the buttons.
        // endregion

        // region Set up the radio buttons.
        for element in items {
            let _radio = button::RadioLightButton::default()
                .with_label(element)
                .with_size(0, 30);
        }
        // endregion

        // region Bring the groups to an end & create the submit button.
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

}

pub mod input_fltk {

/*
// todo: Still getting "unused" warnings.  Fix it if you can figure out how.

*/  // TODO's

use fltk::app::App;
use fltk::{frame, group, input, window};
use fltk::enums::CallbackTrigger;
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