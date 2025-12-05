use fltk::app;
use lib_myfltk::input_fltk::{input_char, input_string};

fn main() {
    let app = app::App::default(); // Ensure App is initialized if not already

    let newstring = input_char(&app, "Try a dry Enter");

    println!("\n The string entered was: {} \n", newstring);

}