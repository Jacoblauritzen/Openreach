// pools.rs - v3

fn fold_pools_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_pools_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct POOLS_3Inner0{val:u64,name:String}
impl POOLS_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
