use crate::xfunc;
use std::{env,fs,io,path::{self, PathBuf}};
use fs2::FileExt;

mod data;

#[derive(Clone,Debug)]
pub enum CaptureError{
    FileError,
    PathError,
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

pub fn tracker_lock(path: &PathBuf)->Result<fs::File,CaptureError>{
    let lock_file_path = path.clone().join("tracker.lock");
    if lock_file_path.exists(){
        println!("{:?} already exists, locking",lock_file_path);
        let lock_file = fs::File::open(lock_file_path)
            .map_err(|_|CaptureError::FileError)?;
        lock_file.try_lock_exclusive().map_err(|_|CaptureError::LockError)?;
        Ok(lock_file)
    }
    else{
        println!("{:?} does not exist creating new", lock_file_path);
        let file = fs::OpenOptions::new().write(true)
                             .create_new(true)
                             .open(lock_file_path)
                            .map_err(|_|CaptureError::FileError)?;
        Ok(file)
    }
}

pub fn check_dir(cc:&CaptureConfig)->Result<fs::File,CaptureError>{
    let p = path::Path::new(&cc.data_path);
    let cpath = fs::canonicalize(p).map_err(|_|CaptureError::PathError)?;
    if !cpath.is_dir() {
        fs::create_dir(&cc.data_path).map_err(|_|CaptureError::PathError)?;
    }
    let lock_file = tracker_lock(&cpath)?;
    Ok(lock_file)
}

pub fn finish_lock(lock_file:fs::File) -> Result<(),CaptureError>{
    lock_file.unlock().map_err(|_|CaptureError::FileError)?;
    Ok(())
}
