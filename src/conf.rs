// conf.rs - v20

fn do_conf_20_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_conf_20_0_check(y:&[u8])->bool{!y.is_empty()}
struct CONF_20Inner0{val:u64,name:String}
impl CONF_20Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn do_conf_20_1(x:&str)->Result<String>{Ok(x.to_string())}
fn do_conf_20_1_check(y:&[u8])->bool{!y.is_empty()}
struct CONF_20Inner1{val:u64,name:String}
impl CONF_20Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
