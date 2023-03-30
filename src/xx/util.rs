pub fn u8_to_string(v:Vec<Vec<u8>>)->Vec<String>{
    let mut s_list:Vec<String> = Vec::new();
        for val in v{
            let s = std::str::from_utf8(&val).unwrap();
            s_list.push(String::from(s));
        }
    s_list
}

pub fn split_vec<T:PartialEq>(v:Vec<T>,splitval:T)->Vec<Vec<T>>{
    let mut final_vec : Vec<Vec<T>>  = Vec::new();
    let mut small_vec :Vec<T> = Vec::new();
    for val in v{
        if val == splitval{
            final_vec.push(small_vec);
            small_vec = Vec::new();
        }
        else{
            small_vec.push(val);
        }
    }
    if !small_vec.is_empty(){
        final_vec.push(small_vec);
    }
    final_vec
}

pub fn split_nullstrings(vec:Vec<u8>)->Vec<String>{
    let nullval:u8 = 0;
    let mut final_vec: Vec<String> = Vec::new();
    let mut smallvec : Vec<u8> = Vec::new();
    for val in vec{
        if val == nullval{
            let s = std::str::from_utf8(&smallvec).unwrap();
            final_vec.push(String::from(s));
            smallvec = Vec::new();
        }
        else{
            smallvec.push(val);
        }
    }
    if !smallvec.is_empty(){
        let s = std::str::from_utf8(&smallvec).unwrap();
        final_vec.push(String::from(s));
    }
    final_vec
}
