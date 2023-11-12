// conf.rs - v6

fn set_conf_6_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_conf_6_0_check(y:&[u8])->bool{!y.is_empty()}
struct CONF_6Inner0{val:u64,name:String}
impl CONF_6Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
