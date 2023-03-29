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
        let a = Atom::new(&self.display,"_NET_DESKTOP_NAMES").unwrap();
        let mut tp = TextProp::default();
        let rwin = self.root_window.unwrap();
        unsafe{
            XGetTextProperty(self.display.0, rwin.0, &mut tp.0, a.0)
        };
        // the textproperty as to be characters
        assert_eq!(tp.format(),8);
        let text = tp.show_metadata().expect("couldn't get textprop metadata");
        let dat = tp.get_data_as(0 as u8).expect("aaa!!");
        util::u8_to_string(dat)
    }
}
