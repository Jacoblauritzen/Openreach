// mailbox.rs - v11

fn set_mailbox_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_mailbox_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_11Inner0{val:u64,name:String}
impl MAILBOX_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
