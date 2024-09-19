use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);

    let mut btn = button::Button::new(160,200,80,30, "CLICK ME");

    win.end();
    win.show();

    btn.set_callback(|_| println!("The button was clicked!"));

    // To do "hover" over the button
    btn.handle(|b, ev| match ev {
        enums::Event::Enter => {
            println!("Entered hover over button.");
            true
        }
        enums::Event::Leave => {
            println!("Left hover over button.");
            true
        }
        _ => false
    });


    a.run().unwrap();
}