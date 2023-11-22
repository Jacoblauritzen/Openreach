// discover.rs - v11

fn do_discover_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_discover_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct DISCOVER_11Inner0{val:u64,name:String}
impl DISCOVER_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
