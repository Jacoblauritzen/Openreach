// inbox.rs - v1

fn run_inbox_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_inbox_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct INBOX_1Inner0{val:u64,name:String}
impl INBOX_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
