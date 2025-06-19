use lib_myfltk::fltkutils::fltk_radio_lightbtn_menu;

fn main() {
    let labels = vec!["flamingo".to_string(), "tiger".to_string(), "lion".to_string()];
    
    let choice = fltk_radio_lightbtn_menu(&labels);
    
    println!("\n choice: {:?} \n", choice);
}