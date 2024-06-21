// send.rs - v20

fn map_send_20_0(x:&str)->Result<String>{Ok(x.to_string())}
fn map_send_20_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_20Inner0{val:u64,name:String}
impl SEND_20Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn run_send_20_1(x:&str)->Result<String>{Ok(x.to_string())}
fn run_send_20_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_20Inner1{val:u64,name:String}
impl SEND_20Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
