// send.rs - v14

fn set_send_14_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_send_14_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_14Inner0{val:u64,name:String}
impl SEND_14Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn fold_send_14_1(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_send_14_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_14Inner1{val:u64,name:String}
impl SEND_14Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
