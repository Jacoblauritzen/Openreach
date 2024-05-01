// user.rs - v9

fn do_user_9_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_user_9_0_check(y:&[u8])->bool{!y.is_empty()}
struct USER_9Inner0{val:u64,name:String}
impl USER_9Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
