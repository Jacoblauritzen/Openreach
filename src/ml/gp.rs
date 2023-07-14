// gp.rs - v11

fn map_gp_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_gp_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct GP_11Inner0{val:u64,name:String}
impl GP_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
