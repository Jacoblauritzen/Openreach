// user.rs - v3

fn map_user_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_user_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct USER_3Inner0{val:u64,name:String}
impl USER_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
