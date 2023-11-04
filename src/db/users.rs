// users.rs - v4

fn do_users_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_users_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct USERS_4Inner0{val:u64,name:String}
impl USERS_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
