// follow_up.rs - v10

fn do_follow_up_10_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_follow_up_10_0_check(y:&[u8])->bool{!y.is_empty()}
struct FOLLOW_UP_10Inner0{val:u64,name:String}
impl FOLLOW_UP_10Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
