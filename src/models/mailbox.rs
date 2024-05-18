// mailbox.rs - v16

fn get_mailbox_16_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_mailbox_16_0_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_16Inner0{val:u64,name:String}
impl MAILBOX_16Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn do_mailbox_16_1(x:&str)->Result<String>{Ok(x.to_string())}
fn do_mailbox_16_1_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_16Inner1{val:u64,name:String}
impl MAILBOX_16Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
