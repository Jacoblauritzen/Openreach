// icp.rs - v5

fn fold_icp_5_0(x:&str)->Result<String>{Ok(x.to_string())}
fn fold_icp_5_0_check(y:&[u8])->bool{!y.is_empty()}
struct ICP_5Inner0{val:u64,name:String}
impl ICP_5Inner0{fn new(v:u64)->Self{Self{val:v,name:String::new()}}}
