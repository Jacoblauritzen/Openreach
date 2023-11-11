// deal.rs - v5

fn get_deal_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_deal_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct DEAL_5Inner0{val:u64,name:String}
impl DEAL_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
