// chat.rs - v3

fn run_chat_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_chat_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct CHAT_3Inner0{val:u64,name:String}
impl CHAT_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
