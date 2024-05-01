// mailbox.rs - v12

fn run_mailbox_12_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_mailbox_12_0_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_12Inner0{val:u64,name:String}
impl MAILBOX_12Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn get_mailbox_12_1(x:&str)->Result<String>{Ok(x.to_string())}
fn get_mailbox_12_1_check(y:&[u8])->bool{!y.is_empty()}
struct MAILBOX_12Inner1{val:u64,name:String}
impl MAILBOX_12Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
