use x11::xlib::{
    XTextProperty,
    XGetTextProperty,
};
use std::{
    ptr::null_mut,
    ffi::CStr,
};

use crate::xx::{
    Atom,
    Display,
    Window,
    TextProp,
    Null,
};

pub struct Session{
    pub display: Display,
    pub root_window: Option<Window>,
}

impl Session{
    pub fn open() -> Result<Self,Null>{
        Ok( Self{
            display: Display::open()?,
            root_window: None,
        } )
    }
    pub fn from_display(display:Display) -> Self{
        Self{
            display,
            root_window:None,
        }
    }
    pub fn set_root_window(&mut self){
        self.root_window = Some(Window::default_root_window(&self.display));
    }
    pub fn get_desktops(&self){
        let a = Atom::new(&self.display,"_NET_DESKTOP_NAMES").unwrap();
        let mut tp = TextProp::default();
        let rwin = self.root_window.unwrap();
        unsafe{
            XGetTextProperty(self.display.0, rwin.0, &mut tp.0, a.0)
        };
        let text = tp.get_metadata().expect("couldn't get textprop metadata");
        let dat = tp.get_data().expect("aaa!!");
        println!("metadata: {}", text);
        println!("data: {}",dat);
    }
}
