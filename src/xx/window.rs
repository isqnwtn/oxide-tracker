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
    slice, fmt::format,
};

use crate::xx::{
    Atom,
    Display,
    TextProp,
};

use super::{X11Error, util};


#[derive(Copy, Clone, Debug)]
pub struct Window(pub XWindow);

#[derive(Clone, Debug)]
pub struct WinProp{
    w_title: Option<String>,
    w_program: Option<String>,
    w_desktop: Option<String>,
}

impl Window{
    pub fn default_root_window(display: &Display) -> Self{
        let win = unsafe{XDefaultRootWindow(display.0)};
        Window(win)
    }
    pub fn windows_from_text_prop(tp: &TextProp) -> Result<Vec<Window>,X11Error>{
        if tp.format() != 32 {return Err(X11Error::UnknownFormat)}
        else{
           let wins : Vec<usize> = tp.get_data_as()?;
           let windows = wins.into_iter().map(|x| Window(x as XWindow)).collect();
            Ok(windows)
        }

    }
    pub fn get_prop(&self,display: &Display)->Result<WinProp,X11Error>{
        // getting window name
        let tpw = TextProp::prop_for_atom(&self, display, "_NET_WM_NAME")?;
        let win_names = tpw.get_data_as()?;
        let mut win_name = util::split_nullstrings(win_names);
        let win_title = if win_name.is_empty(){None}
        else{Some(win_name.remove(0))};

        //program name
        let tpp = TextProp::prop_for_atom(&self, display, "_NET_WM_PID")?;
        let mut pids : Vec<usize> = tpp.get_data_as()?;
        let pid = if pids.is_empty(){None}
        else{util::proc_from_pid(pids.remove(0))};


        let winprop = WinProp{
            w_title: win_title,
            w_program: pid,
            w_desktop: None,
        };
        Ok(winprop)
    }
}
