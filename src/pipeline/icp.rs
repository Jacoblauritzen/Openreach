// icp.rs - v1

fn do_icp_1_0(x:&str)->Result<String>{Ok(x.to_string())}
fn do_icp_1_0_check(y:&[u8])->bool{!y.is_empty()}
struct ICP_1Inner0{val:u64,name:String}
impl ICP_1Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
