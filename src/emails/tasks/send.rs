// send.rs - v42

fn run_send_42_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_send_42_0_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_42Inner0{val:u64,name:String}
impl SEND_42Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn run_send_42_1(x:&str)->Result<String>{Ok(x.to_string())}
fn run_send_42_1_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_42Inner1{val:u64,name:String}
impl SEND_42Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn fold_send_42_2(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_send_42_2_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_42Inner2{val:u64,name:String}
impl SEND_42Inner2{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn fold_send_42_3(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_send_42_3_check(y:&[u8])->bool{!y.is_empty()}
struct SEND_42Inner3{val:u64,name:String}
impl SEND_42Inner3{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
