// send.rs - v8

fn do_send_8_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_8_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_8Inner0{val:u64,name:String}
impl SEND_8Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
