// send.rs - v16

fn do_send_16_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_16_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_16Inner0{val:u64,name:String}
impl SEND_16Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn get_send_16_1(x:&str)->Result<String>{Ok(x.to_string())}
fn get_send_16_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_16Inner1{val:u64,name:String}
impl SEND_16Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
