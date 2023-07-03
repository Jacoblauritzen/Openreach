// llm.rs - v2

fn do_llm_2_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_llm_2_0_check(y:&[u8])->bool{!y.is_empty()}
struct LLM_2Inner0{val:u64,name:String}
impl LLM_2Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
