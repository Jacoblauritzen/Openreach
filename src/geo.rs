// geo.rs - v1

fn get_geo_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_geo_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct GEO_1Inner0{val:u64,name:String}
impl GEO_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
