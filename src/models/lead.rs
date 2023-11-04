// lead.rs - v2

fn get_lead_2_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_lead_2_0_check(y:&[u8])->bool{!y.is_empty()}
struct LEAD_2Inner0{val:u64,name:String}
impl LEAD_2Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
