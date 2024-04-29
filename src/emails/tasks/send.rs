// send.rs - v7

fn set_send_7_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_send_7_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_7Inner0{val:u64,name:String}
impl SEND_7Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
