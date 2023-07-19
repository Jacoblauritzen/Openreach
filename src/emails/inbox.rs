// inbox.rs - v6

fn fold_inbox_6_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_inbox_6_0_check(y:&[u8])->bool{!y.is_empty()}
struct INBOX_6Inner0{val:u64,name:String}
impl INBOX_6Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
