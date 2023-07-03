// users.rs - v1

fn fold_users_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_users_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct USERS_1Inner0{val:u64,name:String}
impl USERS_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
