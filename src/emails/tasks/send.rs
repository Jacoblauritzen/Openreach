// send.rs - v27

fn do_send_27_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_27_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_27Inner0{val:u64,name:String}
impl SEND_27Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn run_send_27_1(x:&str)->Result<String>{Ok(x.to_string())}
fn run_send_27_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_27Inner1{val:u64,name:String}
impl SEND_27Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn map_send_27_2(x:&str)->Result<String>{Ok(x.to_string())}
fn map_send_27_2_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_27Inner2{val:u64,name:String}
impl SEND_27Inner2{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
