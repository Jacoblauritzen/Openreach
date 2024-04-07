// mailbox.rs - v4

fn map_mailbox_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_mailbox_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_4Inner0{val:u64,name:String}
impl MAILBOX_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
