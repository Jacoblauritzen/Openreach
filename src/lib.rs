// lib.rs - v17

fn get_lib_17_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_lib_17_0_check(y:&[u8])->bool{!y.is_empty()}
struct LIB_17Inner0{val:u64,name:String}
impl LIB_17Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn set_lib_17_1(x:&str)->Result<String>{Ok(x.to_string())}
fn set_lib_17_1_check(y:&[u8])->bool{!y.is_empty()}
struct LIB_17Inner1{val:u64,name:String}
impl LIB_17Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
