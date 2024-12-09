
pub mod fltkutils {
    use std::cell::RefCell;
    use std::mem::take;
    use std::rc::Rc;
    use fltk::{app, button, button::Button, group, menu, output, text, window};
    use fltk::app::{quit, set_font_size, App};
    use fltk::enums::{Color, Shortcut};
    use fltk::prelude::{DisplayExt, GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt};
    use fltk::text::{TextBuffer, TextEditor};
    use fltk::window::Window;

    pub fn fltk_chkbox_shift_menu(flist: &Vec<String>) -> Vec<String> {
        let newvec: RefCell<Vec<String>> = RefCell::new(Vec::new());
        let keepers: Rc<RefCell<Vec<String>>> = Rc::new(newvec);

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

        while win.shown() {
            app::wait();
        }

        let retvec: Vec<String> = take(&mut keepers.borrow_mut());
        retvec
    }

    pub fn fltk_radio_lightbtn_menu(flist: &Vec<String>) -> String {

        let newstring: RefCell<String> = RefCell::new("".to_string());
        let keepers: Rc<RefCell<String>> = Rc::new(newstring);

        let mut win = window::Window::default().with_size(400, 300);
        let flex = group::Flex::default().with_size(250, 300);
        let scroll = group::Scroll::default().with_size(200, 200);
        let pack = group::Pack::default().with_size(200, 200);

        for file in flist {
            let _radio = button::RadioLightButton::default()
                .with_label(file)
                .with_size(0, 30);
        }

        pack.end();
        scroll.end();
        flex.end();

        let mut submit = button::Button::new(300, 175, 75, 30, "Submit");

        win.end();
        win.show();

        let keepers_clone = Rc::clone(&keepers);
        let mut win_clone = win.clone();

        submit.set_callback(move |_b| {
            for i in 0..pack.children() {
                let radio: button::RadioLightButton = button::RadioLightButton::from_dyn_widget(&pack.child(i).unwrap()).unwrap();
                if radio.is_toggled() {
                    *keepers_clone.borrow_mut() = radio.label().clone();
                }
            }
            win_clone.hide();
        });

        while win.shown() {
            app::wait();
        }

        let ret: String = keepers.borrow().clone();
        ret
    }

    pub fn fltk_simple_editor(startertxt: &str, winlabel: &str) -> String {
        let edtr = App::default();
        let mut buf = text::TextBuffer::default();
        let mut win = window::Window::default().with_size(800, 300);
        set_font_size(20);
        win.set_color(Color::Blue);
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

        //editor_menubar();

        edtr.run().unwrap();

        buf.text()
    }

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

    pub fn fltk_replace_highlighted_text(edtr: &TextEditor, buf: &mut TextBuffer, rpltxt: &str) {
        let (x, y) = match edtr.buffer().unwrap().selection_position() {
            Some(position) => position,
            None => panic!("\nError!  Could not find a cursor position in the editor.\n"),
        };

        buf.remove(x, y);                         // Remove the selected text
        buf.insert(x, rpltxt);                    // Insert new text and
        edtr.buffer().unwrap().unselect();        // Unhighlight text
    }

    pub fn fltk_popup_2btn(closure1: Box<dyn Fn() + 'static>, label1: &str,
                           closure2: Box<dyn Fn() + 'static>, label2: &str)
    {
        let mut popupwin = Window::default().with_size(575, 100).center_of_parent();

        let mut but1 = Button::new(25, 25, 250, 40, label1);
        let mut but2 = Button::new(300, 25, 250, 40, label2);

        popupwin.end();
        popupwin.show();

        let mut winclone1 = popupwin.clone();
        but1.set_callback(move |_| {
            closure1();
            winclone1.hide();
        });

        let mut winclone2 = popupwin.clone();
        but2.set_callback(move |_| {
            closure2();
            winclone2.hide();
        });


    }

    /*                  Example for using   fltk_popup_2btn()

    fn main() {
    let app = app::App::default();

    let bttn1click = || {
        println!("\n Button 1 was clicked \n");
    };

    let bttn2click = || {
        println!("\n Button 2 was clicked \n");
    };

    fltk_popup_2btn(Box::new(bttn1click), "Button 1",
                    Box::new(bttn2click), "Button 2");

    app.run().unwrap();
}


     */   //Example for using   fltk_popup_2btn()

}  // --------- End   fltkutils   module ----------