// tz_country.rs - v4

fn do_tz_country_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_tz_country_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct TZ_COUNTRY_4Inner0{val:u64,name:String}
impl TZ_COUNTRY_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
