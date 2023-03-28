use x11::xlib::{
    Atom as XAtom,
    False as XFalse,
    XGetWindowProperty,
};
use std::{
    os::raw::{
        c_int,
        c_uchar,
        c_ulong,
    },
    ptr::null_mut,
};
use crate::xfunc::{
    Atom,
    Display,
    NotSupported,
    Window,
};

/// An export of [XGetWindowProperty].
/// Make sure to [x11::xlib::XFree] the pointer, when you're done with it.
///
/// An example of how to handle the response can be found in the [GetWindowPropertyResponse] docs.
pub unsafe fn get_window_property(
    display: &Display,
    window: Window,
    property: Atom,
    expected_type: XAtom
) -> Result<GetWindowPropertyResponse, NotSupported> {
    let mut response = GetWindowPropertyResponse::default();

    if XGetWindowProperty(
        display.0,
        window.0,
        property.0,
        0, 4096 / 4,
        XFalse,
        expected_type,
        &mut response.actual_type_return,
        &mut response.actual_format_return,
        &mut response.nitems_return,
        &mut response.bytes_after_return,
        &mut response.proper_return
    ) == 0 {
        return Ok(response)
    }
    Err(NotSupported)
}

pub struct GetWindowPropertyResponse {
    /// The type of the return.
    pub actual_type_return: XAtom,
    /// The formate of the return whether it is 8, 16 or 32 bytes.
    /// If the architecture is 64-bits and the format is 32,
    /// then the return type wil be 64 bits.
    pub actual_format_return: c_int,
    /// The number of items returned in the lsit.
    pub nitems_return: c_ulong,
    /// The number of bytes that are returned.
    ///
    /// This crate ignores this field.
    pub bytes_after_return: c_ulong,
    /// The pointer that is returned.
    pub proper_return: *mut c_uchar,
}
impl Default for GetWindowPropertyResponse {
    fn default() -> Self {
        Self {
            actual_type_return: 0,
            actual_format_return: 0,
            nitems_return: 0,
            bytes_after_return: 0,
            proper_return: null_mut(),
        }
    }
}
