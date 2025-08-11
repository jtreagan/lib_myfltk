# lib_myfltk
Utility functions that are based on the FLTK.rs crate.
   
## Description
I wrote the functions in this crate to help with other projects on which I am working.  While my main focus has been on my primary project -- Question Bank Creator -- these functions are and will be useful in other situations.  That's why I have kept them in a separate crate.

## Word of Caution
These functions tend to be weak on error handling.  My suggestion is to copy the code into your project and then modify from there.  Better than building from scratch.  I think.  Have fun!

## Library structure
The `lib_myfltk` library has three modules.

   * pub mod fltkutils -- Contains miscellaneous utilities for use with the FLTK-rs GUI.
   * pub mod input_fltk -- Functions for FLTK-rs that use the `fltk::input` module.
   * mod templates -- Functions that need to be modified for use in whatever application they are being used in.  The code is there to be copied as needed and then pasted into the developer's code.

## License
MIT license.

