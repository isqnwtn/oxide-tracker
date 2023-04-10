use std::{fs,collections::HashMap, path::PathBuf, io::{Read, BufReader}};
use serde::{Serialize,Deserialize};

use crate::capture::CaptureError;

pub struct FilePointers{
    meta: fs::File,
}
impl FilePointers{
    pub fn from_path(path:&PathBuf)->Result<FilePointers,CaptureError>{
        let meta_path = path.join("meta.dat");
        let meta_file = fs::File::open(meta_path)
            .map_err(|_|CaptureError::FileError)?;
        let fp = FilePointers{
            meta: meta_file,
        };
        Ok(fp)
    }
}

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
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
        let reader = BufReader::new(&fp.meta);
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
