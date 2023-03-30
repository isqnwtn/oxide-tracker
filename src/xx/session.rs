use x11::xlib::XGetTextProperty;

use crate::xx::{
    Atom,
    Display,
    Window,
    TextProp,
    Null, util,
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
    pub fn get_desktops(&self)->Vec<String>{
        let tp = TextProp::prop_for_atom(&self.display, "_NET_DESKTOP_NAMES");
        // the textproperty as to be characters
        assert_eq!(tp.format(),8);
        let dat = tp.get_data_as(0 as u8).expect("failed to retreive desktop data!!");
        util::u8_to_string(dat)
    }
}
