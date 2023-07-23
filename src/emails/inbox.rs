// inbox.rs - v9

fn map_inbox_9_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_inbox_9_0_check(y:&[u8])->bool{!y.is_empty()}
struct INBOX_9Inner0{val:u64,name:String}
impl INBOX_9Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
