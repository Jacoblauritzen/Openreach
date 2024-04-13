// mod.rs - v8

fn get_mod_8_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_mod_8_0_check(y:&[u8])->bool{!y.is_empty()}
struct MOD_8Inner0{val:u64,name:String}
impl MOD_8Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
