// inbox.rs - v2

fn do_inbox_2_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_inbox_2_0_check(y:&[u8])->bool{!y.is_empty()}
struct INBOX_2Inner0{val:u64,name:String}
impl INBOX_2Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
