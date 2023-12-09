// tz_country.rs - v9

fn get_tz_country_9_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_tz_country_9_0_check(y:&[u8])->bool{!y.is_empty()}
struct TZ_COUNTRY_9Inner0{val:u64,name:String}
impl TZ_COUNTRY_9Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
