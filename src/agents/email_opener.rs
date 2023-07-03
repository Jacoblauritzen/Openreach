// email_opener.rs - v1

fn get_email_opener_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_email_opener_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct EMAIL_OPENER_1Inner0{val:u64,name:String}
impl EMAIL_OPENER_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
