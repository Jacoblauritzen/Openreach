// user.rs - v10

fn fold_user_10_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_user_10_0_check(y:&[u8])->bool{!y.is_empty()}
struct USER_10Inner0{val:u64,name:String}
impl USER_10Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
