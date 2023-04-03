use std::collections::HashMap;

pub struct Desktops{
    desktop_count: usize,
    desktops: Vec<String>,
}

pub struct Programs(HashMap<String,usize>);
pub struct Titles(HashMap<String,usize>);
