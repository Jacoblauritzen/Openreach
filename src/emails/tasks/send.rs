// send.rs - v28

fn do_send_28_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_send_28_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_28Inner0{val:u64,name:String}
impl SEND_28Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn fold_send_28_1(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_send_28_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_28Inner1{val:u64,name:String}
impl SEND_28Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn get_send_28_2(x:&str)->Result<String>{Ok(x.to_string())}
fn get_send_28_2_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_28Inner2{val:u64,name:String}
impl SEND_28Inner2{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
