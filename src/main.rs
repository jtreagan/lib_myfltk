use fltk::app;

use lib_myfltk::fltkutils::fltk_custom_message;

fn main() {
    let app = app::App::default(); // Ensure App is initialized if not already

    fltk_custom_message("That folder is empty.", "Return to Main Menu");
}