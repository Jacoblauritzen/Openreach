// mod.rs - v11

fn do_mod_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_mod_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct MOD_11Inner0{val:u64,name:String}
impl MOD_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
