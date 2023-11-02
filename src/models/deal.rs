// deal.rs - v2

fn run_deal_2_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_deal_2_0_check(y:&[u8])->bool{!y.is_empty()}
struct DEAL_2Inner0{val:u64,name:String}
impl DEAL_2Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
