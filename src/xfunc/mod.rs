mod display;
mod window;
mod atom;
mod session;
//helpers
pub mod util;

pub use self::{
    atom::Atom,
    session::Session,
    display::Display,
    window::Window,
};

/// A struct which is used to represent that an error occured due to a Null pointer.
#[derive(Copy, Clone, Debug)]
pub struct Null;

/// A struct that represents an error where the ``_NET_ClIENT_LIST`` property
/// was not found in the root window.
///
/// This error can be caused by using Desktop Environments that does not support
/// the above convention.
/// The WMCTRL tool's source code that I used as a reference to make this crate
/// checked for another property, if the first one didn't work,
/// but as I had no need for it I didn't implement it.
/// But if there is a need for it I should have no problem implementing that as well.
///
/// Another possible source of this error was that the size of the item was not expected.
///
/// If this error happens please make an issue on the GitHub repo,
/// giving the OS; architecture; and/or desktop environment; of your computer.
#[derive(Copy, Clone, Debug)]
pub struct NotSupported;

const NET_CLIENT_LIST: &str = "_NET_CLIENT_LIST";

const NET_ACTIVE_WINDOW: &str = "_NET_ACTIVE_WINDOW";

use x11::xlib;
use std::{ptr::{null,null_mut}, ffi::CStr};

pub fn test(){
    unsafe{
        let display = xlib::XOpenDisplay(null());
        if display == null_mut(){
            panic!("Can't open display")
        }
        let rootwindow = xlib::XDefaultRootWindow(display);


        let mut prop_struct:xlib::XTextProperty =  xlib::XTextProperty{
            value: null_mut(),
            encoding: 0,
            format: 0,
            nitems: 0,
        };

        xlib::XGetWMName(display, rootwindow , &mut prop_struct);
        if !prop_struct.value.is_null(){
            let text = CStr::from_ptr(prop_struct.value as *mut i8);
            let rust_string = String::from_utf8_lossy(text.to_bytes()).to_string();
            println!("window name {}",rust_string);
        }
        else {
            println!("window name turned out to be null");
        }

        println!("default root window: {}", rootwindow);
    }
}

use std::{thread,time};
pub fn test2(){
   let mut session = Session::open()
    .expect("Could not open session");
    for _x in 0..10{
        let window = session.active_window().expect("couldn't get active window");
        let title = window.get_title(&session.display).expect("couldnt get title");
        println!("{:?}",title);
        let five_sec = time::Duration::from_secs(5);
        thread::sleep(five_sec);
    }
}
