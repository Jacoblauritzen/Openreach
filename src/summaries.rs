// summaries.rs - v1

fn do_summaries_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_summaries_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct SUMMARIES_1Inner0{val:u64,name:String}
impl SUMMARIES_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
