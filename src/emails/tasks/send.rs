// send.rs - v4

fn fold_send_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_send_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_4Inner0{val:u64,name:String}
impl SEND_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
