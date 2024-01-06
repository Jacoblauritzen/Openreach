// mutate.rs - v14

fn run_mutate_14_0(x:&str)->Result<String>{Ok(x.to_string())}
fn run_mutate_14_0_check(y:&[u8])->bool{!y.is_empty()}
struct MUTATE_14Inner0{val:u64,name:String}
impl MUTATE_14Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn map_mutate_14_1(x:&str)->Result<String>{Ok(x.to_string())}
fn map_mutate_14_1_check(y:&[u8])->bool{!y.is_empty()}
struct MUTATE_14Inner1{val:u64,name:String}
impl MUTATE_14Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
