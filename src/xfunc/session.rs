use x11::xlib::{
    Window as XWindow,
    XA_WINDOW,
    XFree,
};
use std::{
    os::raw::c_void,
    slice,
};
use crate::xfunc::{
    Atom,
    Display,
    NET_CLIENT_LIST,
    NotSupported,
    Null,
    Window,
};

pub struct Session {
    /// A display that has been opened.
    pub display: Display,
    /// The root window of the display.
    pub root_window: Option<Window>,
    /// The atom that represents the client_list property.
    pub client_list_atom: Option<Atom>,
    /// The atom that represents the active_window property.
    pub active_window_atom: Option<Atom>,
}

impl Session{
    pub fn open() -> Result<Self, Null>{
        Ok( Self{
            display: Display::open()?,
            root_window: None,
            client_list_atom: None,
            active_window_atom: None,
        } )
    }

    pub fn from_display(display: Display) -> Self {
        Self {
            display,
            root_window: None,
            client_list_atom: None,
            active_window_atom: None,
        }
    }

    /// Gets the currently active window in the display.
    pub fn active_window(&mut self) -> Result<Window, NotSupported> {
        Window::active_window(self)
    }
}
