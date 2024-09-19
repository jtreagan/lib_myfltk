
/*                            Goals

    -- Done!!!

*/


use lib_file::file_fltk::file_pathonly;
use lib_file::file_mngmnt::file_get_dir_list;
use lib_myfltk::fltkutils::chkbox_shift_menu;

fn main() {

    let path = file_pathonly();
    println!("The chosen path is:  {} \n", path);

    let flist = file_get_dir_list(&path);
    println!("The files in that directory are:  {:?} \n", flist);
    
    let fnamesvctr = chkbox_shift_menu( &flist);

    println!("The vector of chosen file names is:  {:?} \n", fnamesvctr);


}

