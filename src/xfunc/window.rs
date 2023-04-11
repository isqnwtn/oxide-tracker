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

use crate::xfunc::{
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
    w_desktop: Option<usize>,
    w_active: bool,
}
impl WinProp{
    pub fn is_active(&self)->Option<(String,String,usize)>{
        if !self.w_active{
           None
        }
        else{
            let title = self.w_title.clone()?;
            let prog = self.w_program.clone()?;
            let desk = self.w_desktop?;
            Some((title,prog,desk))
        }
    }
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
    pub fn get_prop(&self,display: &Display,awin:&usize)->Result<WinProp,X11Error>{
        // getting window name
        let tpw = TextProp::prop_for_atom(&self, display, "_NET_WM_NAME")?;
        let win_names = tpw.get_data_as()?;
        let mut win_name = util::split_nullstrings(win_names);
        let win_title = if win_name.is_empty(){None}
        else{Some(win_name.remove(0))};

        //program name
        // TODO: Implement this using XGetClassHint
        let tpp = TextProp::prop_for_atom(&self, display, "_NET_WM_PID")?;
        let pid = tpp.get_single_prop()?;
        let progname = util::proc_from_pid(pid);

        //desktop
        let tpd = TextProp::prop_for_atom(&self, display, "_NET_WM_DESKTOP")?;
        let wdesk : usize = tpd.get_single_prop()?;

        let winprop = WinProp{
            w_title: win_title,
            w_program: progname,
            w_desktop: Some(wdesk),
            w_active: (*awin as u64 == self.0)
        };
        Ok(winprop)
    }
}
