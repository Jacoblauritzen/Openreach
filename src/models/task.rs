// task.rs - v11

fn set_task_11_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_task_11_0_check(y:&[u8])->bool{!y.is_empty()}
struct TASK_11Inner0{val:u64,name:String}
impl TASK_11Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
