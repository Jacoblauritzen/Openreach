// ready_pool.rs - v3

fn set_ready_pool_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_ready_pool_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct READY_POOL_3Inner0{val:u64,name:String}
impl READY_POOL_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
