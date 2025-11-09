

use lib_myfltk::input_fltk::*;

fn main() {
    let app = fltk::app::App::default();

    //let acceptable = vec!["yes", "Yes", "y", "Y", "no", "No", "n", "N", "true", "t", "false", "f", "True", "T","False", "F"];

    let choice = input_truefalse(&app, "Enter yes or no");
    
    println!("\n choice: {} \n", choice);
    
    app.run().unwrap();
}