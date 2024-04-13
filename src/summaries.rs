// summaries.rs - v5

fn set_summaries_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_summaries_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct SUMMARIES_5Inner0{val:u64,name:String}
impl SUMMARIES_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
