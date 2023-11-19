// descend.rs - v7

fn do_descend_7_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_descend_7_0_check(y:&[u8])->bool{!y.is_empty()}
struct DESCEND_7Inner0{val:u64,name:String}
impl DESCEND_7Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
