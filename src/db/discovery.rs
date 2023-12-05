// discovery.rs - v9

fn fold_discovery_9_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_discovery_9_0_check(y:&[u8])->bool{!y.is_empty()}
struct DISCOVERY_9Inner0{val:u64,name:String}
impl DISCOVERY_9Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
