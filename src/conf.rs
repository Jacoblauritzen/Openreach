// conf.rs - v8

fn map_conf_8_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_conf_8_0_check(y:&[u8])->bool{!y.is_empty()}
struct CONF_8Inner0{val:u64,name:String}
impl CONF_8Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
