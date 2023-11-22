// ready_pool.rs - v5

fn do_ready_pool_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_ready_pool_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct READY_POOL_5Inner0{val:u64,name:String}
impl READY_POOL_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
