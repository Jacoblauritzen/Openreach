// bettercontact.rs - v5

fn get_bettercontact_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_bettercontact_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct BETTERCONTACT_5Inner0{val:u64,name:String}
impl BETTERCONTACT_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
