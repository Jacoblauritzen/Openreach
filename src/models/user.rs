// user.rs - v5

fn do_user_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_user_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct USER_5Inner0{val:u64,name:String}
impl USER_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
