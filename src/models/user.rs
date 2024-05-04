// user.rs - v11

fn set_user_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_user_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct USER_11Inner0{val:u64,name:String}
impl USER_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
