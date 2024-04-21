// mailbox.rs - v7

fn do_mailbox_7_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_mailbox_7_0_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_7Inner0{val:u64,name:String}
impl MAILBOX_7Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
