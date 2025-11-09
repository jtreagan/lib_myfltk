

use lib_myfltk::input_fltk::*;

fn main() {
    let app = fltk::app::App::default();

    let choice = input_truefalse(&app, "Enter yes or no");
    
    println!("\n choice: {} \n", choice);
    
    app.run().unwrap();
}