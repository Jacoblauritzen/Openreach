// smtp.rs - v9

fn get_smtp_9_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_smtp_9_0_check(y:&[u8])->bool{!y.is_empty()}
struct SMTP_9Inner0{val:u64,name:String}
impl SMTP_9Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
