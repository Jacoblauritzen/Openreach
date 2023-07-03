// deals.rs - v1

fn get_deals_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_deals_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct DEALS_1Inner0{val:u64,name:String}
impl DEALS_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
