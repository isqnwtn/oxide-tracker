use std::{path, time::{self, SystemTime}, thread};

use capture::MetaData;

use crate::capture::CaptureError;

use chrono::{DateTime,Utc};

mod xfunc;
mod capture;

macro_rules!  skip_fail{
    ($res:expr) => {
       match $res {
           Some(val) => val,
           None => continue,
       }
    };
}


fn main()->Result<(),capture::CaptureError>{

    // setting up the configuration and files
    let config = capture::CaptureConfig::new("./data",10);
    let (new,lockfile) = capture::check_dir(&config)?;
    let mut fp = capture::FilePointers::from_config(&config)?;

    // setting up the environment
    let wait = time::Duration::from_secs(config.get_samplerate() as u64);
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

    // infinite loop
    loop {
        let now: DateTime<Utc> = Utc::now();
        let windows = session.get_client_list()
            .map_err(|x|CaptureError::ReadError(x))?;
        for w in windows{
            let (t,p,d) = skip_fail!(w.is_active());
            println!("found: {} => {} - {} - {}",now,t,p,d);
            let mut changed = metadata.add_pgm(&p);
            changed = metadata.add_title(&t) || changed;
            if changed{
                println!("saving!!");
                metadata.save_changes(&mut fp)
                    .map_err(|_|CaptureError::FileError)?;
                break;
            }
        }
        thread::sleep(wait);
    }

    // the code wouldn't reach here lol
    capture::finish_lock(lockfile)?;
    Ok(())
}
