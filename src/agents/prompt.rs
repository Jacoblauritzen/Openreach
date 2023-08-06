// prompt.rs - v11

fn set_prompt_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_prompt_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct PROMPT_11Inner0{val:u64,name:String}
impl PROMPT_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
