// summaries.rs - v3

fn get_summaries_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_summaries_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct SUMMARIES_3Inner0{val:u64,name:String}
impl SUMMARIES_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
