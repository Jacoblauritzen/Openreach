// onboarding.rs - v5

fn set_onboarding_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_onboarding_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct ONBOARDING_5Inner0{val:u64,name:String}
impl ONBOARDING_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
