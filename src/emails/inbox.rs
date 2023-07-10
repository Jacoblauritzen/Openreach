// inbox.rs - v3

fn set_inbox_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_inbox_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct INBOX_3Inner0{val:u64,name:String}
impl INBOX_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
