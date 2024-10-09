
/*                            Goals

    -- Done!!!

*/


use lib_file::file_fltk::file_pathonly;
use lib_file::file_mngmnt::file_get_dir_list;
use lib_myfltk::fltkutils::{chkbox_shift_menu, radio_lightbtn_menu};

fn main() {




    let path = file_pathonly();
    let flist = file_get_dir_list(&path);

    let fname = radio_lightbtn_menu( &flist);

    println!("\n The chosen file name is:  {} \n", fname);
}

