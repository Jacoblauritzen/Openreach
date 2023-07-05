// prompts.rs - v3

fn fold_prompts_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_prompts_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct PROMPTS_3Inner0{val:u64,name:String}
impl PROMPTS_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
