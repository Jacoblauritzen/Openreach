// discovery.rs - v15

fn get_discovery_15_0(x:&str)->Result<String>{Ok(x.to_string())}
fn get_discovery_15_0_check(y:&[u8])->bool{!y.is_empty()}
struct DISCOVERY_15Inner0{val:u64,name:String}
impl DISCOVERY_15Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}

fn set_discovery_15_1(x:&str)->Result<String>{Ok(x.to_string())}
fn set_discovery_15_1_check(y:&[u8])->bool{!y.is_empty()}
struct DISCOVERY_15Inner1{val:u64,name:String}
impl DISCOVERY_15Inner1{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
