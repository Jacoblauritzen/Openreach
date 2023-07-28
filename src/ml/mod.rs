// mod.rs - v16

fn set_mod_16_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_mod_16_0_check(y:&[u8])->bool{!y.is_empty()}
struct MOD_16Inner0{val:u64,name:String}
impl MOD_16Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn get_mod_16_1(x:&str)->Result<String>{Ok(x.to_string())}
fn get_mod_16_1_check(y:&[u8])->bool{!y.is_empty()}
struct MOD_16Inner1{val:u64,name:String}
impl MOD_16Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
