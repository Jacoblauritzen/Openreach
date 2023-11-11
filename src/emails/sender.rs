// sender.rs - v3

fn get_sender_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_sender_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct SENDER_3Inner0{val:u64,name:String}
impl SENDER_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
