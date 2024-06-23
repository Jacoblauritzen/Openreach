// send.rs - v21

fn do_send_21_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_21_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_21Inner0{val:u64,name:String}
impl SEND_21Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn run_send_21_1(x:&str)->Result<String>{Ok(x.to_string())}
fn run_send_21_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_21Inner1{val:u64,name:String}
impl SEND_21Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
