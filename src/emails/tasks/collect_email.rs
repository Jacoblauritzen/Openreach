// collect_email.rs - v3

fn do_collect_email_3_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_collect_email_3_0_check(y:&[u8])->bool{!y.is_empty()}
struct COLLECT_EMAIL_3Inner0{val:u64,name:String}
impl COLLECT_EMAIL_3Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
