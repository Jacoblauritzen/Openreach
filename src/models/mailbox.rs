// mailbox.rs - v15

fn run_mailbox_15_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_mailbox_15_0_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_15Inner0{val:u64,name:String}
impl MAILBOX_15Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn do_mailbox_15_1(x:&str)->Result<String>{Ok(x.to_string())}
fn do_mailbox_15_1_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_15Inner1{val:u64,name:String}
impl MAILBOX_15Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
