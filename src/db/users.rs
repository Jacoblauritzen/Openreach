// users.rs - v5

fn map_users_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_users_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct USERS_5Inner0{val:u64,name:String}
impl USERS_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
