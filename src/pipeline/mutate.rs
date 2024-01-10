// mutate.rs - v15

fn fold_mutate_15_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_mutate_15_0_check(y:&[u8])->bool{!y.is_empty()}
struct MUTATE_15Inner0{val:u64,name:String}
impl MUTATE_15Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn set_mutate_15_1(x:&str)->Result<String>{Ok(x.to_string())}
fn set_mutate_15_1_check(y:&[u8])->bool{!y.is_empty()}
struct MUTATE_15Inner1{val:u64,name:String}
impl MUTATE_15Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
