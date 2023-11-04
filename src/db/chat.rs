// chat.rs - v4

fn map_chat_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_chat_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct CHAT_4Inner0{val:u64,name:String}
impl CHAT_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
