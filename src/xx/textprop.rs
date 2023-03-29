use x11::xlib::{
    XTextProperty,
    XFree,
};

use std::{
    ptr::null_mut, ffi::CStr,
    os::raw::c_void,
    slice,
};

use crate::xx::{
    util,
    Null
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
    pub fn format(&self)->usize{
        self.0.format as usize
    }
    pub fn show_metadata(&self) -> Result<String,Null> {
        if !self.0.value.is_null(){
            let text = unsafe{CStr::from_ptr(self.0.value as *mut i8)};
            Ok(format!("enoding:{} format:{} nitems:{}"
                       ,self.0.encoding,self.0.format,self.0.nitems))
        }
        else {
            Err(Null)
        }
    }
    pub fn get_data_as<T:Clone+PartialEq>(&self,nullval:T) -> Result<Vec<Vec<T>>,Null>{
        if !self.0.value.is_null(){
            let val = unsafe{slice::from_raw_parts(
               self.0.value as *const T, self.0.nitems as usize
            )};
            let vector = val.to_vec();
            let final_vec = util::split_vec(vector, nullval);
            Ok(final_vec)
        }
        else{
            Err(Null)
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
