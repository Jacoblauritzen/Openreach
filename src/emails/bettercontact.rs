// bettercontact.rs - v3

fn map_bettercontact_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_bettercontact_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct BETTERCONTACT_3Inner0{val:u64,name:String}
impl BETTERCONTACT_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
