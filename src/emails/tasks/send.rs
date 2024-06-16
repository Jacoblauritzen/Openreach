// send.rs - v18

fn do_send_18_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_18_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_18Inner0{val:u64,name:String}
impl SEND_18Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn set_send_18_1(x:&str)->Result<String>{Ok(x.to_string())}
fn set_send_18_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_18Inner1{val:u64,name:String}
impl SEND_18Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
