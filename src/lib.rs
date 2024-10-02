#![warn(unused_imports)]


pub mod fltkutils {
    use std::cell::RefCell;
    use std::mem::take;
    use std::rc::Rc;
    use fltk::{app, button, group, output, window};
    use fltk::prelude::{GroupExt, InputExt, WidgetBase, WidgetExt};
    //use lib_jt::vec::*;

    pub fn chkbox_shift_menu(flist: &Vec<String>) -> Vec<String> {
        let newvec: RefCell<Vec<String>> = RefCell::new(Vec::new());
        let keepers: Rc<RefCell<Vec<String>>> = Rc::new(newvec);

        let app = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
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

        let mut btn = button::Button::default().with_label("@>");
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

        app.run().unwrap();

        let retvec: Vec<String> = take(&mut keepers.borrow_mut());
        retvec
    }



    pub fn radio_lightbtn_menu(flist: &Vec<String>) -> Vec<String> {

        let newstring: RefCell<String> = RefCell::new("".to_string());
        let keepers: Rc<RefCell<String>> = Rc::new(newstring);

        let app = app::App::default();
        let mut win = window::Window::default().with_size(400, 300);
        let mut row = group::Flex::default_fill().row();
        let scroll = group::Scroll::default();
        row.fixed(&scroll, 150);
        let pack = group::Pack::default().with_size(100, 300);

        for file in flist {
            let _check = button::RadioLightButton::default()
                .with_label(file)
                .with_size(0, 30);
        }

        pack.end();
        scroll.end();

        let mut btn = button::Button::default().with_label("@>");
        row.fixed(&btn, 30);
        let mut output = output::MultilineOutput::default();

        row.end();
        win.end();
        win.show();

        let mut keepers_clone = Rc::clone(&keepers);
        btn.set_callback(move |_b| {
            output.set_value("");
            let mut string = String::new();
            for i in 0..pack.children() {
                let radio: button::RadioLightButton = button::RadioLightButton::
                from_dyn_widget(&pack.child(i).unwrap()).unwrap();
                if radio.is_toggled() {
                    string.push_str(&radio.label());
                    //string.push('\n');
                    keepers_clone = radio.label().clone();
                }
            }
            output.set_value(&string);
        });

        app.run().unwrap();

        let retvec: Vec<String> = take(&mut keepers.borrow_mut());
        retvec
    }


    /*  First attempt at single-item return

    pub fn radio_lightbtn_menu(list: &Vec<String>)  {
        //let newvec: RefCell<Vec<String>> = RefCell::new(Vec::new());
        //let keepers: Rc<RefCell<Vec<String>>> = Rc::new(newvec);

        let app = app::App::default();
        let mut win = window::Window::default().with_size(200, 300);
        let mut flex = group::Flex::default_fill();
        let scroll = group::Scroll::default();
        let mut pack = group::Pack::default().with_size(200,300);

        for fname in list {
            let _radlite = button::RadioLightButton::default()
                .with_label(fname)
                .with_size(75, 50);
        }

        pack.end();
        scroll.end();

        // Sets up the  Submit  button
        let mut btn = button::Button::default().with_label("@Submit");
        flex.fixed(&btn, 50);
        //let mut output = output::MultilineOutput::default();

        flex.end();
        win.end();
        win.show();

        /*
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

         */

        app.run().unwrap();

        //let retvec: Vec<String> = take(&mut keepers.borrow_mut());
        //retvec
    }

     */


}  // --------- End   fltkutils   module ----------