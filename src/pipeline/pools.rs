// pools.rs - v8

fn set_pools_8_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_pools_8_0_check(y:&[u8])->bool{!y.is_empty()}
struct POOLS_8Inner0{val:u64,name:String}
impl POOLS_8Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
