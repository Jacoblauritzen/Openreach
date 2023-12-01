// mutate.rs - v4

fn map_mutate_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_mutate_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct MUTATE_4Inner0{val:u64,name:String}
impl MUTATE_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
