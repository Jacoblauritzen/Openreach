// main.rs - v11

fn set_main_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_main_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct MAIN_11Inner0{val:u64,name:String}
impl MAIN_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
