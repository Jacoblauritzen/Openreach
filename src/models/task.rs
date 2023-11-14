// task.rs - v5

fn get_task_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_task_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct TASK_5Inner0{val:u64,name:String}
impl TASK_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
