// lib.rs - v5

fn run_lib_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_lib_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct LIB_5Inner0{val:u64,name:String}
impl LIB_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
