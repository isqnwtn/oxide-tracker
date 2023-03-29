use x11::xlib::{
    XTextProperty,
    XFree,
};

use std::{
    ptr::null_mut, ffi::CStr,
    os::raw::c_void,
    slice,
};

use crate::xx::Null;

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
    pub fn get_metadata(&self) -> Result<String,Null> {
        if !self.0.value.is_null(){
            let text = unsafe{CStr::from_ptr(self.0.value as *mut i8)};
            Ok(format!("value:{:?} enoding:{} format:{} nitems:{}"
                       ,text,self.0.encoding,self.0.format,self.0.nitems))
        }
        else {
            Err(Null)
        }
    }
    pub fn get_data(&self) -> Result<String,Null>{
        if !self.0.value.is_null(){
            let val = match self.0.format{
                8 => {
                    unsafe{slice::from_raw_parts(
                    self.0.value as *const i8, self.0.nitems as usize )
                    }
                }
                16 => {panic!("aaaa!!")}
                32 => {panic!("aaaaa!!")}
                _ => {panic!("aaaaa!!")}
            };
            Ok(format!("{:?}",val))
        }
        else{
            Err(Null)
        }
    }

}

impl Drop for TextProp{
    fn drop(&mut self){
        unsafe{ XFree(self.0.value as *mut c_void) };
    }
}
