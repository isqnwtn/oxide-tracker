mod xfunc;
mod capture;


fn main()->Result<(),capture::CaptureError>{
    let config = capture::CaptureConfig::new("./data",1);
    let dd = capture::check_dir(&config)?;
    xfunc::test();
    capture::finish_lock(dd)?;
    Ok(())
}
