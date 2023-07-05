// gp.rs - v3

fn get_gp_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_gp_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct GP_3Inner0{val:u64,name:String}
impl GP_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
