// leads.rs - v10

fn do_leads_10_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_leads_10_0_check(y:&[u8])->bool{!y.is_empty()}
struct LEADS_10Inner0{val:u64,name:String}
impl LEADS_10Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
