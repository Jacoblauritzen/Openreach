// follow_up.rs - v5

fn run_follow_up_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_follow_up_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct FOLLOW_UP_5Inner0{val:u64,name:String}
impl FOLLOW_UP_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
