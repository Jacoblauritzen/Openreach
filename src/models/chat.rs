// chat.rs - v6

fn do_chat_6_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_chat_6_0_check(y:&[u8])->bool{!y.is_empty()}
struct CHAT_6Inner0{val:u64,name:String}
impl CHAT_6Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
