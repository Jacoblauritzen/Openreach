// tz_country.rs - v10

fn set_tz_country_10_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_tz_country_10_0_check(y:&[u8])->bool{!y.is_empty()}
struct TZ_COUNTRY_10Inner0{val:u64,name:String}
impl TZ_COUNTRY_10Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
