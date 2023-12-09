// task.rs - v10

fn do_task_10_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_task_10_0_check(y:&[u8])->bool{!y.is_empty()}
struct TASK_10Inner0{val:u64,name:String}
impl TASK_10Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
