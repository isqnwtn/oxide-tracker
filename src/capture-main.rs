use std::path;

use capture::MetaData;


mod xfunc;
mod capture;


fn main()->Result<(),capture::CaptureError>{

    // setting up the configuration and files
    let config = capture::CaptureConfig::new("./data",5);
    let (new,lockfile) = capture::check_dir(&config)?;
    let mut fp = capture::FilePointers::from_config(&config)?;

    //xfunc::test();
    let mut session = xfunc::Session::open().unwrap();
    session.set_root_window();
    let desks = session.get_desktops().unwrap();
    let mut metadata = if new{
        let mut md = capture::MetaData::empty();
        md.set_desks(desks);
        println!("creating new metadata");
        md.save_changes(&mut fp).unwrap();
        md
    }
    else{
        println!("loading metadata");
        MetaData::load_from_file(&mut fp).unwrap()
    };
    metadata.save_changes(&mut fp).unwrap();
    println!("metadata: {:?}",metadata);

    // the code wouldn't reach here lol
    capture::finish_lock(lockfile)?;
    Ok(())
}
