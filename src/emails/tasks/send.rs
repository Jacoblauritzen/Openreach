// send.rs - v32

fn run_send_32_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_send_32_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_32Inner0{val:u64,name:String}
impl SEND_32Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn do_send_32_1(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_32_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_32Inner1{val:u64,name:String}
impl SEND_32Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn do_send_32_2(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_32_2_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_32Inner2{val:u64,name:String}
impl SEND_32Inner2{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
