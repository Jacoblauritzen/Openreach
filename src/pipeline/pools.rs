// pools.rs - v11

fn fold_pools_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_pools_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct POOLS_11Inner0{val:u64,name:String}
impl POOLS_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
