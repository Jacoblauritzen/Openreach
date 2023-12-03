// chat.rs - v11

fn run_chat_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_chat_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct CHAT_11Inner0{val:u64,name:String}
impl CHAT_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
