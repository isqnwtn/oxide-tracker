mod display;
mod atom;
mod window;
mod session;
mod textprop;

pub use self::{
    atom::Atom,
    display::Display,
    window::Window,
    session::Session,
    textprop::TextProp,
};

/// A struct which is used to represent that an error occured due to a Null pointer.
#[derive(Copy, Clone, Debug)]
pub struct Null;


pub fn test(){
    println!("testing the x11 functions..");
    let mut session = Session::open().expect("couldn't open session");
    session.set_root_window();
    session.get_desktops();
}
