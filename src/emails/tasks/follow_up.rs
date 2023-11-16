// follow_up.rs - v7

fn do_follow_up_7_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_follow_up_7_0_check(y:&[u8])->bool{!y.is_empty()}
struct FOLLOW_UP_7Inner0{val:u64,name:String}
impl FOLLOW_UP_7Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
