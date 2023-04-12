use chrono::{DateTime,Utc};

#[derive(Debug)]
pub struct StoreData {
    time: DateTime<Utc>,
    desk: u8,
    program: usize,
    title: usize,
}

impl StoreData {
    pub fn to_bytes(&self)->[u8;12]{
        unimplemented!()
    } 
    pub fn from_bytes(line:&[u8;12])->Result<StoreData,std::io::Error>{
        unimplemented!()
    }
}
