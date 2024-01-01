// mutate.rs - v13

fn do_mutate_13_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_mutate_13_0_check(y:&[u8])->bool{!y.is_empty()}
struct MUTATE_13Inner0{val:u64,name:String}
impl MUTATE_13Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn run_mutate_13_1(x:&str)->Result<String>{Ok(x.to_string())}
fn run_mutate_13_1_check(y:&[u8])->bool{!y.is_empty()}
struct MUTATE_13Inner1{val:u64,name:String}
impl MUTATE_13Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
