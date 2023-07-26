// inbox.rs - v12

fn do_inbox_12_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_inbox_12_0_check(y:&[u8])->bool{!y.is_empty()}
struct INBOX_12Inner0{val:u64,name:String}
impl INBOX_12Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn run_inbox_12_1(x:&str)->Result<String>{Ok(x.to_string())}
fn run_inbox_12_1_check(y:&[u8])->bool{!y.is_empty()}
struct INBOX_12Inner1{val:u64,name:String}
impl INBOX_12Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
