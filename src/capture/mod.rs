use crate::xfunc;
mod metadata;
mod file;
mod data;

pub use self::{
    file::check_dir,
    file::finish_lock,
    metadata::FilePointers,
    metadata::MetaData,
    data::StoreData,
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
    pub fn get_samplerate(&self)->usize{
        self.sample_rate
    }
}
