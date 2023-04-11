use std::{fs,path::{self, PathBuf}};
use fs2::FileExt;

use crate::capture::{
    CaptureConfig,
    CaptureError,
};

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
        let lock_file = fs::OpenOptions::new().write(true)
                             .create_new(true)
                             .open(lock_file_path)
                            .map_err(|_|CaptureError::FileError)?;
        Ok(lock_file)
    }
}

macro_rules!  create_with_err{
    ($path:ident) => {
        fs::create_dir(&($path))
            .map_err(|_|CaptureError::PathError(String::from(
                format!("cannot create path {:?}",$path)
            )))
    };
    ($path:ident,$file:literal) => {
        fs::File::create(&($path.join($file)))
            .map_err(|_|CaptureError::PathError(String::from(
                format!("Cannot create file {:?}", $path.join($file))
            )))
    }
}

pub fn check_dir(cc:&CaptureConfig)->Result<(bool,fs::File),CaptureError>{
    let p = path::Path::new(&cc.data_path);
    // if the directory is being created for the first time.
    let mut new = false;
    if !p.is_dir() {
        create_with_err!(p)?;
        let meta = p.join("meta");
        let data = p.join("data");
        create_with_err!(meta)?;
        create_with_err!(data)?;
        create_with_err!(meta,"meta.dat")?;
        new = true;
    }
    let cpath = fs::canonicalize(p)
        .map_err(|_|CaptureError::PathError(
            String::from("cannot cannonicalize path")
        ))?;
    let lock_file = tracker_lock(&cpath)?;
    Ok((new,lock_file))
}

pub fn finish_lock(lock_file:fs::File) -> Result<(),CaptureError>{
    lock_file.unlock().map_err(|_|CaptureError::FileError)?;
    Ok(())
}
