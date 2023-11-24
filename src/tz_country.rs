// tz_country.rs - v3

fn run_tz_country_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_tz_country_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct TZ_COUNTRY_3Inner0{val:u64,name:String}
impl TZ_COUNTRY_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
