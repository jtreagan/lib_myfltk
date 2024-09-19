
use fltk::{prelude::*, *};

fn main() {
    
    chkboxmenu();
    
}

fn chkboxmenu() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let flex = group::Flex::default().with_size(100, 200).column().center_of_parent();

    let mut btn1 = button::CheckButton::default().with_label("Option 1");
    btn1.set_value(true);
    let btn2 = button::CheckButton::default().with_label("Option 2");
    let mut btn3 = button::Button::default().with_label("Submit");

    flex.end();
    win.end();
    win.show();

    btn3.set_callback(move |btn3| {
        if btn1.value() {
            println!("Button 1 has been checked.");
        }
        if btn2.value() {
            println!("Button 2 has been checked.");
        }
    });

    a.run().unwrap();
}
