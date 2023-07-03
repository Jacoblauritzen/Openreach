// descend.rs - v1

fn fold_descend_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_descend_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct DESCEND_1Inner0{val:u64,name:String}
impl DESCEND_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
