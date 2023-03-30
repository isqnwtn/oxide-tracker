use x11::xlib::XGetTextProperty;

use crate::xx::{
    Atom,
    Display,
    Window,
    WinProp,
    TextProp,
    util,
    X11Error
};


pub struct Session{
    pub display: Display,
    pub root_window: Option<Window>,
}

impl Session{
    pub fn open() -> Result<Self,X11Error>{
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
    pub fn get_desktops(&self)->Result<Vec<String>,X11Error>{
        let win = self.root_window.ok_or(X11Error::Unset)?;
        let tp = TextProp::prop_for_atom(&win,&self.display, "_NET_DESKTOP_NAMES")?;
        // the textproperty as to be characters
        if tp.format() != 8 {return Err(X11Error::UnknownFormat)}
        let dat : Vec<u8> = tp.get_data_as()?;
        Ok(util::split_nullstrings(dat))
    }
    pub fn get_client_list(&self)->Result<Vec<WinProp>,X11Error>{
        let win = self.root_window.ok_or(X11Error::Unset)?;
        let tp = TextProp::prop_for_atom(&win,&self.display, "_NET_CLIENT_LIST")?;
        // the textproperty has to be 32bits
        if tp.format() != 32 {return Err(X11Error::UnknownFormat)}
        let windows = Window::windows_from_text_prop(&tp)?;
        let wins : Vec<WinProp> = windows
            .iter()
            .filter_map(|x| x.get_prop(&self.display).ok())
            .collect();
        Ok(wins)
    }
}
