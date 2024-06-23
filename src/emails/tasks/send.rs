// send.rs - v23

fn do_send_23_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_23_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_23Inner0{val:u64,name:String}
impl SEND_23Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn set_send_23_1(x:&str)->Result<String>{Ok(x.to_string())}
fn set_send_23_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_23Inner1{val:u64,name:String}
impl SEND_23Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
