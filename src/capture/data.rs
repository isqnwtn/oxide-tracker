use std::{fs,collections::HashMap, path::PathBuf, io::Read};
use serde::{Serialize,Deserialize};

use super::CaptureError;

pub struct Desktops{
    desktop_count: usize,
    desktops: Vec<String>,
}

impl Desktops{
    pub fn empty()->Desktops{
        Desktops { desktop_count: 0, desktops: Vec::new() }
    }
    pub fn from(v:Vec<String>)->Desktops{
       Desktops { desktop_count: v.len(), desktops: v }
    }
}

pub struct Programs{
    pgm_count: usize,
    pgm_map:HashMap<String,usize>,
}

impl Programs{
    pub fn empty()->Programs{
        Programs { pgm_count: 0, pgm_map: HashMap::new() }
    }
    pub fn from(v:Vec<(String,usize)>)->Programs{
        Programs { pgm_count: v.len(), pgm_map: v.into_iter().collect() }
    }
}

pub struct Titles{
    title_count: usize,
    title_map:HashMap<String,usize>,
}

impl Titles{
    pub fn empty()->Titles{
        Titles { title_count: 0, title_map: HashMap::new() }
    }
    pub fn from(v:Vec<(String,usize)>)->Titles{
        Titles { title_count: v.len(), title_map: v.into_iter().collect() }
    }
}

pub struct FilePointers{
    deskpgm: fs::File,
    title: fs::File,
}
impl FilePointers{
    pub fn from_path(path:&PathBuf)->Result<FilePointers,CaptureError>{
        let deskpgm_path = path.join("deskpgm.dat");
        let title_path = path.join("titles.dat");
        let deskpgm_file = fs::File::open(deskpgm_path)
            .map_err(|_|CaptureError::FileError)?;
        let title_file = fs::File::open(title_path)
            .map_err(|_|CaptureError::FileError)?;
        let fp = FilePointers{
            deskpgm: deskpgm_file,
            title: title_file,
        };
        Ok(fp)
    }
}


#[derive(Serialize,Deserialize,PartialEq,Debug)]
pub struct SaveInfoDeskPgm{
    save_desk : Vec<String>,
    save_pgm : Vec<(String,usize)>,
}

#[derive(Serialize,Deserialize,PartialEq,Debug)]
pub struct SaveInfoTitle{
    save_title: Vec<(String,usize)>
}



pub struct CaptureData{
    desk_dat: Desktops,
    pgm_dat: Programs,
    title_dat: Titles,
    changed: bool,
}
impl CaptureData{
    pub fn empty()->CaptureData{
        CaptureData
        { desk_dat: Desktops::empty(),
          pgm_dat: Programs::empty(),
          title_dat: Titles::empty(),
          changed: false,
        }
    }
    pub fn load_from_file(fp: &mut FilePointers)->Result<CaptureData,std::io::Error>{
        let mut deskpgm_str = String::new();
        let mut title_str = String::new();
        fp.deskpgm.read_to_string(&mut deskpgm_str)?;
        fp.title.read_to_string(&mut title_str)?;
        let save_info_deskpgm  = serde_json::from_str::<SaveInfoDeskPgm>(&deskpgm_str)?;
        let save_info_title = serde_json::from_str::<SaveInfoTitle>(&title_str)?;
        Ok(CaptureData {
            desk_dat: Desktops::from(save_info_deskpgm.save_desk),
            pgm_dat: Programs::from(save_info_deskpgm.save_pgm),
            title_dat: Titles::from(save_info_title.save_title),
            changed: false,
        })
    }
    pub fn save_to_file(&self,fp: &mut FilePointers)->Result<(),std::io::Error>{
        if !self.changed{
            Ok(())
        }
        else{
            Ok(())
        }
    }
}
