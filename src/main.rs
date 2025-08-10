

use lib_myfltk::fltkutils::*;


fn main() {
    let app = fltk::app::App::default();
    
    let labels = vec!["flamingo".to_string(), "tiger".to_string(), "lion".to_string()];
    
    let choice = fltk_checkbox_menu(app.clone(), &labels);
    
    println!("\n choice: {:?} \n", choice);
    
    app.run().unwrap();
}