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
