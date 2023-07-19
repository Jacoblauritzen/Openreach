// gp.rs - v15

fn set_gp_15_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_gp_15_0_check(y:&[u8])->bool{!y.is_empty()}
struct GP_15Inner0{val:u64,name:String}
impl GP_15Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn map_gp_15_1(x:&str)->Result<String>{Ok(x.to_string())}
fn map_gp_15_1_check(y:&[u8])->bool{!y.is_empty()}
struct GP_15Inner1{val:u64,name:String}
impl GP_15Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
