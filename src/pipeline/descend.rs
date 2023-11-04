// descend.rs - v2

fn do_descend_2_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_descend_2_0_check(y:&[u8])->bool{!y.is_empty()}
struct DESCEND_2Inner0{val:u64,name:String}
impl DESCEND_2Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
