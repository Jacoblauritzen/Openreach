// conf.rs - v15

fn run_conf_15_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_conf_15_0_check(y:&[u8])->bool{!y.is_empty()}
struct CONF_15Inner0{val:u64,name:String}
impl CONF_15Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn do_conf_15_1(x:&str)->Result<String>{Ok(x.to_string())}
fn do_conf_15_1_check(y:&[u8])->bool{!y.is_empty()}
struct CONF_15Inner1{val:u64,name:String}
impl CONF_15Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
