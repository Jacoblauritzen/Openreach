// gp.rs - v16

fn set_gp_16_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_gp_16_0_check(y:&[u8])->bool{!y.is_empty()}
struct GP_16Inner0{val:u64,name:String}
impl GP_16Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn get_gp_16_1(x:&str)->Result<String>{Ok(x.to_string())}
fn get_gp_16_1_check(y:&[u8])->bool{!y.is_empty()}
struct GP_16Inner1{val:u64,name:String}
impl GP_16Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
