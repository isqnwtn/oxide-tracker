use x11::xlib::{
    XGetTextProperty,
    XTextProperty,
    XFree,
};

use std::{
    ptr::null_mut, ffi::CStr,
    os::raw::c_void,
    slice,
};

use crate::xfunc::{
    Window,
    Display,
    Atom,
    util,
    X11Error
};

pub struct TextProp(pub XTextProperty);

impl TextProp{
    pub fn default()->Self{
        TextProp(
            XTextProperty{
                value: null_mut(),
                encoding: 0,
                format: 0,
                nitems: 0,
            }
        )
    }
    pub fn from_prop(prop: XTextProperty) -> Self{
        TextProp(prop)
    }
    pub fn prop_for_atom(win: &Window,display: &Display,atom_str:&str)->Result<TextProp,X11Error>{
        let a = Atom::new(display,atom_str)?;
        let mut tp = TextProp::default();
        unsafe{
            XGetTextProperty(display.0, win.0, &mut tp.0, a.0)
        };
        Ok(tp)
    }
    pub fn format(&self)->usize{
        self.0.format as usize
    }
    pub fn show_metadata(&self) -> Result<String,X11Error> {
        if !self.0.value.is_null(){
            Ok(format!("enoding:{} format:{} nitems:{}"
                       ,self.0.encoding,self.0.format,self.0.nitems))
        }
        else {
            Err(X11Error::Invalid)
        }
    }
    pub fn get_data_as<T:Clone+PartialEq>(&self) -> Result<Vec<T>,X11Error>{
        if !self.0.value.is_null(){
            let val = unsafe{slice::from_raw_parts(
               self.0.value as *const T, self.0.nitems as usize
            )};
            let vector = val.to_vec();
            Ok(vector)
        }
        else{
            Err(X11Error::ParseError)
        }
    }
    pub fn get_single_prop<T:Clone+PartialEq+Copy>(&self) -> Result<T,X11Error>{
        if !self.0.value.is_null(){
            let val = unsafe{slice::from_raw_parts(
                self.0.value as *const T, self.0.nitems as usize)};
            let vec = val.to_vec();
            let first = vec.first().ok_or(X11Error::ParseError)?;
            Ok(*first)
        }
        else{
            Err(X11Error::ParseError)
        }
    }

}

impl Drop for TextProp{
    fn drop(&mut self){
        if !self.0.value.is_null(){
            unsafe{ XFree(self.0.value as *mut c_void) };
        }
    }
}
