// prompt.rs - v7

fn fold_prompt_7_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_prompt_7_0_check(y:&[u8])->bool{!y.is_empty()}
struct PROMPT_7Inner0{val:u64,name:String}
impl PROMPT_7Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
