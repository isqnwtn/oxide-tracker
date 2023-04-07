use crate::xfunc;
mod data;
mod file;

pub use self::{
    file::check_dir,
    file::finish_lock,
};

#[derive(Clone,Debug)]
pub enum CaptureError{
    FileError,
    PathError(String),
    LockError,
    ReadError(xfunc::X11Error),
}


pub struct CaptureConfig{
    data_path: String, // the path to the directory where data is stored
    sample_rate: usize, // sampling rate in seconds
}
impl CaptureConfig{
    pub fn new(path:&str,rate:usize)->CaptureConfig{
        CaptureConfig{
            data_path: String::from(path),
            sample_rate: rate,
        }
    }
}

