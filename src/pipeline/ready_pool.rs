// ready_pool.rs - v11

fn get_ready_pool_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_ready_pool_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct READY_POOL_11Inner0{val:u64,name:String}
impl READY_POOL_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
