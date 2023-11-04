// task.rs - v3

fn set_task_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_task_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct TASK_3Inner0{val:u64,name:String}
impl TASK_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
