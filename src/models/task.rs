// task.rs - v9

fn map_task_9_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_task_9_0_check(y:&[u8])->bool{!y.is_empty()}
struct TASK_9Inner0{val:u64,name:String}
impl TASK_9Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
