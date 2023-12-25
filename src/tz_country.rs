// tz_country.rs - v11

fn do_tz_country_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_tz_country_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct TZ_COUNTRY_11Inner0{val:u64,name:String}
impl TZ_COUNTRY_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
