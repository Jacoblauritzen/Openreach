// geo.rs - v7

fn get_geo_7_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_geo_7_0_check(y:&[u8])->bool{!y.is_empty()}
struct GEO_7Inner0{val:u64,name:String}
impl GEO_7Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
