// session.rs - v12

fn set_session_12_0(x:&str)->Result<String>{Ok(x.to_string())}
fn set_session_12_0_check(y:&[u8])->bool{!y.is_empty()}
struct SESSION_12Inner0{val:u64,name:String}
impl SESSION_12Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn map_session_12_1(x:&str)->Result<String>{Ok(x.to_string())}
fn map_session_12_1_check(y:&[u8])->bool{!y.is_empty()}
struct SESSION_12Inner1{val:u64,name:String}
impl SESSION_12Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
