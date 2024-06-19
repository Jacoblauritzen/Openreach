// send.rs - v19

fn run_send_19_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_send_19_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_19Inner0{val:u64,name:String}
impl SEND_19Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn get_send_19_1(x:&str)->Result<String>{Ok(x.to_string())}
fn get_send_19_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_19Inner1{val:u64,name:String}
impl SEND_19Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
