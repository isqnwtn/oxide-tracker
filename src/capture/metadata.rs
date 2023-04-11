use std::{fs,collections::HashMap, path::{PathBuf, self}, io::{Read, BufReader}};
use serde::{Serialize,Deserialize};

use crate::capture::CaptureError;

use super::CaptureConfig;

pub struct FilePointers{
    meta: fs::File,
    meta_read: fs::File,
    meta_path: PathBuf,
}
impl FilePointers{
    pub fn from_config(cc:&CaptureConfig)->Result<FilePointers,CaptureError>{
        let path = path::Path::new(&cc.data_path);
        let meta_path = path.join("meta").join("meta.dat");

        let meta_file = fs::File::options()
            .read(true)
            .write(true)
            .open(&meta_path)
            .map_err(|_|CaptureError::FileError)?;

        let meta_read_file = fs::File::open(&meta_path)
            .map_err(|_|CaptureError::FileError)?;

        let fp = FilePointers{
            meta: meta_file,
            meta_read: meta_read_file,
            meta_path: meta_path,
        };
        Ok(fp)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TwoHash{
    size: usize,
    fw: HashMap<String,usize>,
    bw: HashMap<usize,String>,
}
impl TwoHash{
    pub fn new()->TwoHash{
       TwoHash { size: 0, fw: HashMap::new(), bw: HashMap::new() }
    }
    pub fn capacity(&self)->usize{
        self.size
    }
    pub fn fw_exists(&self,name:&str)->bool{
       self.fw.contains_key(name)
    }
    pub fn fw_lookup(&self,name:&str)->Option<&usize>{
        self.fw.get(name)
    }
    pub fn bw_lookup(&self,id:&usize)->Option<&String>{
        self.bw.get(id)
    }
    pub fn bw_exists(&self,id:&usize)->bool{
        self.bw.contains_key(&id)
    }
    // returns true if the addition was successfull, false otherwise
    pub fn add(&mut self,name:&str)->bool{
       let new_id = self.size + 1;
       if !self.fw_exists(name){
           self.fw.insert(String::from(name), new_id);
           self.bw.insert(new_id, String::from(name));
           self.size = self.size +1;
           true
       }
        else{
            false
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct MetaData{
    desk_dat: Vec<String>,
    pgm_dat: TwoHash,
    title_dat: TwoHash,
}
impl MetaData{
    pub fn empty()->MetaData{
        MetaData
        { desk_dat: Vec::new(),
          pgm_dat: TwoHash::new(),
          title_dat: TwoHash::new(),
        }
    }
    pub fn load_from_file(fp: &mut FilePointers)->Result<MetaData,serde_json::Error>{
        let file = fs::File::open(&fp.meta_path).expect("cant open meta file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
    }
    pub fn save_changes(&mut self,fp: &mut FilePointers)->Result<(),serde_json::Error>{
        serde_json::to_writer(&fp.meta, &self)
    }
    pub fn set_desks(&mut self,desks: Vec<String>){
        self.desk_dat = desks;
    }
    pub fn add_pgm(&mut self,pgm: &str)->bool{
        self.pgm_dat.add(pgm)
    }
    pub fn add_title(&mut self,title: &str)->bool{
        self.title_dat.add(title)
    }
}
