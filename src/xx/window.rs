use x11::xlib::{
    Window as XWindow,
    XA_WINDOW,
    XDefaultRootWindow,
    XFree,
    XGetWMName,
    XTextProperty,
};
use std::{
    ffi::CStr,
    ops::Drop,
    os::raw::c_void,
    ptr::null_mut,
    slice,
};

use crate::xx::{
    Atom,
    Display,
    TextProp,
};

use super::X11Error;


#[derive(Copy, Clone, Debug)]
pub struct Window(pub XWindow);

impl Window{
    pub fn default_root_window(display: &Display) -> Self{
        let win = unsafe{XDefaultRootWindow(display.0)};
        Window(win)
    }
    pub fn windows_from_text_prop(tp: &TextProp) -> Result<Vec<Window>,X11Error>{
        todo!()
    }
}
