// mod.rs - v5

fn fold_mod_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_mod_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct MOD_5Inner0{val:u64,name:String}
impl MOD_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
