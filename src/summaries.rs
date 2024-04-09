// summaries.rs - v4

fn run_summaries_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_summaries_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct SUMMARIES_4Inner0{val:u64,name:String}
impl SUMMARIES_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
