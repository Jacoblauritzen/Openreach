// sender.rs - v4

fn do_sender_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_sender_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct SENDER_4Inner0{val:u64,name:String}
impl SENDER_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
