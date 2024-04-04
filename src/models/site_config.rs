// site_config.rs - v5

fn set_site_config_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_site_config_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct SITE_CONFIG_5Inner0{val:u64,name:String}
impl SITE_CONFIG_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
