// chat.rs - v1

fn fold_chat_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_chat_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct CHAT_1Inner0{val:u64,name:String}
impl CHAT_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
