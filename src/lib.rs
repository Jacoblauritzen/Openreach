// lib.rs - v13

fn map_lib_13_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_lib_13_0_check(y:&[u8])->bool{!y.is_empty()}
struct LIB_13Inner0{val:u64,name:String}
impl LIB_13Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn set_lib_13_1(x:&str)->Result<String>{Ok(x.to_string())}
fn set_lib_13_1_check(y:&[u8])->bool{!y.is_empty()}
struct LIB_13Inner1{val:u64,name:String}
impl LIB_13Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
