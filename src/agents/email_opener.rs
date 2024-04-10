// email_opener.rs - v4

fn do_email_opener_4_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_email_opener_4_0_check(y:&[u8])->bool{!y.is_empty()}
struct EMAIL_OPENER_4Inner0{val:u64,name:String}
impl EMAIL_OPENER_4Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
